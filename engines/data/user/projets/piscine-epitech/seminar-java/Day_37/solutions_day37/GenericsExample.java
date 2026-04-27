import java.util.List;
import java.util.ArrayList;

/**
 * Démonstration des Génériques Java
 * 
 * Sans génériques : le compilateur ne détecte pas les erreurs de type
 * Avec génériques : le compilateur détecte les erreurs à la compilation
 */
public class GenericsExample {
    
    public static void main(String[] args) {
        System.out.println("=== EXEMPLE 1 : SANS GÉNÉRIQUES (DANGEREUX) ===");
        demonstrateWithoutGenerics();
        
        System.out.println("\n=== EXEMPLE 2 : AVEC GÉNÉRIQUES (TYPE-SAFE) ===");
        demonstrateWithGenerics();
    }
    
    /**
     * Exemple DANGEREUX : sans génériques
     * Le compilateur ne peut pas détecter l'erreur de type
     */
    public static void demonstrateWithoutGenerics() {
        // Attention : List sans spécification de type = RAW TYPE
        List list = new ArrayList();
        
        list.add("test");  // Ajoute une String
        list.add(42);      // Ajoute un Integer
        
        // ⚠️ PROBLÈME : On essaie de caster une String en Integer
        // Cette ligne compile, mais jettera une ClassCastException à l'exécution !
        try {
            Integer i = (Integer) list.get(0); // Runtime Error !!!
            System.out.println("i = " + i); // Jamais exécuté
        } catch (ClassCastException e) {
            System.out.println("❌ ERREUR RUNTIME : " + e.getMessage());
            System.out.println("   Problème : Tentative de cast de String vers Integer");
            System.out.println("   Ce type d'erreur n'est détecté qu'à l'exécution !");
        }
    }
    
    /**
     * Exemple SÛR : avec génériques
     * Le compilateur détecte l'erreur à la compilation
     */
    public static void demonstrateWithGenerics() {
        // ✅ Type-safe : Le compilateur sait que cette liste contient des String
        List<String> stringList = new ArrayList<String>();
        
        stringList.add("test");
        stringList.add("hello");
        // stringList.add(42); // ❌ ERREUR DE COMPILATION ! Impossible d'ajouter un Integer
        // Le compilateur refuse cette ligne et on évite l'erreur runtime
        
        String str = stringList.get(0); // ✅ Pas besoin de cast, type garanti
        System.out.println("str = " + str);
        System.out.println("✅ Tout fonctionne correctement !");
        
        // Exemple avec Integer
        List<Integer> intList = new ArrayList<Integer>();
        intList.add(42);
        intList.add(1337);
        
        Integer num = intList.get(0); // ✅ Pas besoin de cast
        System.out.println("num = " + num);
        
        // ❌ Cette ligne causerait une erreur de compilation :
        // Integer error = intList.get(0); // Si on essayait de caster en Integer alors que c'est String
        // Mais heureusement, c'est impossible car intList ne contient que des Integer !
    }
}

