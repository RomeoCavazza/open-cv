import json
import os
import glob
import math
import time
import numpy as np
from numba import cuda, float32, int32, int16

# --- CONFIGURATION BLACKWELL ---
# Optimisation pour RTX 5070 Ti : Grand nombre de threads pour masquer la latence
TPB = 256  # Threads Per Block

# Caractéristiques Antennes [Portée, Capacité, Coût_Sur_Bat, Coût_Hors_Bat]
# Index: 0=Nano, 1=Spot, 2=Density, 3=MaxRange
ANTENNA_SPECS_HOST = np.array([
    [50,   200,   5000,   6000],
    [100,  800,   15000,  20000],
    [150,  5000,  30000,  50000],
    [400,  3500,  40000,  50000]
], dtype=np.float32)

# --- NOYAUX CUDA (GOD TIER) ---

@cuda.jit(device=True)
def get_dist_sq(x1, y1, x2, y2):
    return (x1 - x2)**2 + (y1 - y2)**2

@cuda.jit
def greedy_solver_kernel(buildings, antenna_specs, coverage_mask, antenna_result_type, antenna_result_root):
    """
    Approche Gloutonne Parallèle avec Priorité Spatiale.
    Chaque thread s'occupe d'un bâtiment. S'il n'est pas couvert, il tente d'ouvrir une antenne sur LUI-MÊME
    (car c'est moins cher) et de capturer les voisins.
    """
    idx = cuda.grid(1)
    if idx >= buildings.shape[0]:
        return

    # Si le bâtiment est déjà couvert par un thread précédent (race condition acceptée pour la vitesse), on saute
    if coverage_mask[idx] == 1:
        return

    # Coordonnées du bâtiment courant (candidat pour être une antenne)
    my_id = int(buildings[idx, 0])
    my_x = buildings[idx, 1]
    my_y = buildings[idx, 2]
    
    # Stratégie : On essaie de placer l'antenne ici.
    # On doit décider QUEL type d'antenne.
    # Pour l'instant, on tente le plus petit type qui couvre la demande locale, 
    # puis on upgrade si on peut attraper des voisins.
    
    # Simplification "Speedrun": On prend une antenne "Spot" (Type 1) par défaut 
    # car c'est souvent le meilleur ratio coût/capacité pour les zones denses.
    # Sauf si la demande locale exige plus.
    
    # 1. Calcul de la demande locale (juste moi)
    my_pop = buildings[idx, 3]
    
    best_type = -1
    # Trouver le plus petit type capable de me supporter
    for t in range(4):
        if antenna_specs[t, 1] >= my_pop:
            best_type = t
            break
            
    if best_type == -1: return # Ne devrait pas arriver
    
    # 2. Tentative de capture des voisins (Greedy Capture)
    # On regarde les bâtiments suivants dans la liste (qui est triée spatialement !)
    # Cela permet de grouper sans faire un N*N complet.
    
    current_load = my_pop
    max_cap = antenna_specs[best_type, 1]
    range_sq = antenna_specs[best_type, 0] ** 2
    
    # On marque que JE suis couvert par MOI-MÊME
    # Utilisation d'atomicCAS pour éviter que deux antennes se montent dessus
    if cuda.atomic.compare_and_swap(coverage_mask, idx, 0, 1) == 0:
        antenna_result_type[idx] = best_type
        antenna_result_root[idx] = idx # Je suis ma propre racine
        
        # Scan des voisins (Lookahead limité pour perf)
        # Sur Blackwell, on peut se permettre un loop de 500-1000 sans trop de soucis grâce au cache L1
        lookahead = 2000 
        
        for k in range(1, lookahead):
            neighbor_idx = idx + k
            if neighbor_idx >= buildings.shape[0]:
                break
                
            if coverage_mask[neighbor_idx] == 1:
                continue
                
            n_x = buildings[neighbor_idx, 1]
            n_y = buildings[neighbor_idx, 2]
            n_pop = buildings[neighbor_idx, 3]
            
            dist = get_dist_sq(my_x, my_y, n_x, n_y)
            
            if dist <= range_sq:
                if current_load + n_pop <= max_cap:
                    # On tente de capturer ce voisin
                    if cuda.atomic.compare_and_swap(coverage_mask, neighbor_idx, 0, 1) == 0:
                        current_load += n_pop
                        antenna_result_root[neighbor_idx] = idx # Il est rattaché à moi
                        antenna_result_type[neighbor_idx] = -2 # Marqué comme "enfant"
                else:
                    # Si on est plein, on regarde si upgrader l'antenne vaut le coup
                    # (Logique simplifiée pour la version v1: on s'arrête là)
                    pass

# --- ORCHESTRATION PYTHON ---

def solve_dataset(file_path):
    print(f"\n Traitement de {file_path}...")
    
    # 1. Chargement & Parsing
    with open(file_path, 'r') as f:
        data = json.load(f)
        
    raw_buildings = []
    for b in data['buildings']:
        pop = max(b['populationPeakHours'], b['populationOffPeakHours'], b['populationNight'])
        raw_buildings.append([b['id'], b['x'], b['y'], pop])
    
    # Conversion Numpy
    buildings = np.array(raw_buildings, dtype=np.float32)
    n = buildings.shape[0]
    print(f"    {n} bâtiments chargés.")

    # 2. TRI SPATIAL (CRITIQUE pour la performance Cache L2)
    # On trie par X puis par Y. Cela place les bâtiments géographiquement proches
    # à des index proches en mémoire.
    # Pour le Dataset 5, c'est vital.
    sort_indices = np.lexsort((buildings[:, 2], buildings[:, 1])) # Sort by Y then X
    buildings_sorted = buildings[sort_indices]
    
    # Mapping inverse pour retrouver les ID originaux à la fin
    original_ids = buildings_sorted[:, 0].astype(np.int32)
    
    # 3. Préparation GPU
    d_buildings = cuda.to_device(buildings_sorted)
    d_specs = cuda.to_device(ANTENNA_SPECS_HOST)
    
    # Tableaux de résultats
    # coverage_mask: 0=Non, 1=Oui
    d_coverage = cuda.device_array(n, dtype=np.int32) 
    # antenna_result_type: Type d'antenne (0-3) si racine, -1 si rien, -2 si enfant
    d_ant_type = cuda.to_device(np.full(n, -1, dtype=np.int32))
    # antenna_result_root: Index (dans sorted) de l'antenne qui couvre ce batiment
    d_ant_root = cuda.to_device(np.full(n, -1, dtype=np.int32))
    
    # Initialisation à 0
    cuda.memset_d32(d_coverage, 0, n)

    # 4. Exécution "God Tier"
    threads = TPB
    blocks = (n + threads - 1) // threads
    
    start_gpu = time.time()
    greedy_solver_kernel[blocks, threads](
        d_buildings, d_specs, d_coverage, d_ant_type, d_ant_root
    )
    cuda.synchronize()
    end_gpu = time.time()
    
    print(f"   ⚡ GPU Compute Time: {(end_gpu - start_gpu)*1000:.2f} ms")
    
    # 5. Récupération & Post-Processing
    res_type = d_ant_type.copy_to_host()
    res_root = d_ant_root.copy_to_host()
    
    # On doit reconstruire le JSON. 
    # On itère sur res_type. Si > -1, c'est une antenne active.
    # On cherche tous ceux qui pointent vers elle dans res_root.
    
    # Optimisation Python avec Numpy pour éviter les boucles lentes
    final_antennas = []
    
    # Trouver les indices des racines (là où on a posé des antennes)
    root_indices = np.where(res_type >= 0)[0]
    
    antenna_names = ["Nano", "Spot", "Density", "MaxRange"]
    
    total_cost = 0
    
    # On crée un dictionnaire inversé : Root_Index -> [List of Child Indices]
    # C'est la partie lente en Python pur, on l'accélère
    import pandas as pd
    df = pd.DataFrame({'root': res_root, 'original_id': original_ids})
    # On ne garde que ceux qui sont couverts (root != -1)
    df = df[df['root'] != -1]
    grouped = df.groupby('root')['original_id'].apply(list).to_dict()
    
    for root_idx in root_indices:
        t_id = res_type[root_idx]
        if root_idx not in grouped: continue # Antenne sans clients ? Rare mais possible
        
        assigned_buildings = grouped[root_idx]
        
        # Coordonnées de l'antenne (sur le bâtiment root)
        ax = int(buildings_sorted[root_idx, 1])
        ay = int(buildings_sorted[root_idx, 2])
        
        # Calcul coût (Sur batiment)
        cost = ANTENNA_SPECS_HOST[t_id, 2]
        total_cost += cost
        
        final_antennas.append({
            "type": antenna_names[t_id],
            "x": ax,
            "y": ay,
            "buildings": assigned_buildings
        })
        
    print(f"    Coût estimé : {total_cost:,.0f} €")
    print(f"    Nombre d'antennes : {len(final_antennas)}")
    
    # 6. Sauvegarde
    output_data = {"antennas": final_antennas}
    os.makedirs("solutions", exist_ok=True)
    out_name = os.path.basename(file_path).replace("input", "solution")
    out_path = os.path.join("solutions", out_name)
    
    with open(out_path, 'w') as f:
        json.dump(output_data, f, indent=4)
        
    print(f"   ✅ Sauvegardé: {out_path}")

def main():
    # Trouve tous les fichiers input dans le dossier courant ou sous-dossiers
    inputs = glob.glob("**/*input*.json", recursive=True)
    if not inputs:
        print("❌ Aucun fichier input*.json trouvé !")
        return
        
    print(f" Démarrage GOD TIER SOLVER sur {cuda.get_current_device().name}")
    
    for inp in inputs:
        try:
            solve_dataset(inp)
        except Exception as e:
            print(f"❌ Erreur sur {inp}: {e}")
            import traceback
            traceback.print_exc()

if __name__ == "__main__":
    main()