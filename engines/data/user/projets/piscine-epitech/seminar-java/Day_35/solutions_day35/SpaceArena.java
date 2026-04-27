import java.util.*;

public class SpaceArena {
    private List<Monster> monsters;
    private List<SpaceMarine> spaceMarines;

    public SpaceArena() {
        this.monsters = new ArrayList<>();
        this.spaceMarines = new ArrayList<>();
    }

    public void enlistMonsters(List<Monster> monsterList) {
        for (Monster monster : monsterList) {
            if (!monsters.contains(monster)) {
                monsters.add(monster);
            }
        }
    }

    public void enlistSpaceMarines(List<SpaceMarine> marineList) {
        for (SpaceMarine marine : marineList) {
            if (!spaceMarines.contains(marine)) {
                spaceMarines.add(marine);
            }
        }
    }

    public boolean fight() {
        if (monsters.isEmpty()) {
            System.out.println("No monsters available to fight.");
            return false;
        }
        if (spaceMarines.isEmpty()) {
            System.out.println("Those cowards ran away.");
            return false;
        }

        List<Monster> availableMonsters = new ArrayList<>(monsters);
        List<SpaceMarine> availableMarines = new ArrayList<>(spaceMarines);
        
        Monster currentMonster = availableMonsters.remove(0);
        SpaceMarine currentMarine = availableMarines.remove(0);
        
        System.out.println(currentMarine.getName() + " has entered the arena.");
        System.out.println(currentMonster.getName() + " has entered the arena.");

        while (!availableMonsters.isEmpty() && !availableMarines.isEmpty()) {
            Fighter winner = fightRound(currentMarine, currentMonster);
            
            if (winner instanceof SpaceMarine) {
                winner.recoverAP();
                currentMonster = availableMonsters.remove(0);
                System.out.println(currentMonster.getName() + " has entered the arena.");
            } else {
                winner.recoverAP();
                currentMarine = availableMarines.remove(0);
                System.out.println(currentMarine.getName() + " has entered the arena.");
            }
        }
        if (availableMonsters.isEmpty()) {
            System.out.println("The spaceMarines are victorious.");
        } else {
            System.out.println("The monsters are victorious.");
        }
        
        return true;
    }

    private Fighter fightRound(Fighter fighter1, Fighter fighter2) {
        while (fighter1.getHp() > 0 && fighter2.getHp() > 0) {
            if (!tryAttack(fighter1, fighter2)) {
                if (fighter1.getAp() < getRequiredAP(fighter1)) {
                    fighter1.recoverAP();
                } else {
                    fighter1.moveCloseTo(fighter2);
                }
            }
            
            if (fighter2.getHp() <= 0) break;
            
            if (!tryAttack(fighter2, fighter1)) {
                if (fighter2.getAp() < getRequiredAP(fighter2)) {
                    fighter2.recoverAP();
                } else {
                    fighter2.moveCloseTo(fighter1);
                }
            }
            
            if (fighter1.getHp() <= 0) break;
        }
        
        return (fighter1.getHp() > 0) ? fighter1 : fighter2;
    }

    private boolean tryAttack(Fighter attacker, Fighter target) {
        if (attacker instanceof SpaceMarine) {
            SpaceMarine marine = (SpaceMarine) attacker;
            if (marine.getWeapon() != null && marine.getWeapon().isMelee() && marine.getCloseTarget() != target) {
                return false;
            }
            if (marine.getWeapon() != null && marine.getAp() < marine.getWeapon().getApcost()) {
                return false;
            }
        } else if (attacker instanceof Monster) {
            Monster monster = (Monster) attacker;
            if (monster.getCloseTarget() != target) {
                return false;
            }
            if (monster.getAp() < monster.getApcost()) {
                return false;
            }
        }
        
        return attacker.attack(target);
    }

    private int getRequiredAP(Fighter fighter) {
        if (fighter instanceof SpaceMarine) {
            SpaceMarine marine = (SpaceMarine) fighter;
            return (marine.getWeapon() != null) ? marine.getWeapon().getApcost() : 0;
        } else if (fighter instanceof Monster) {
            Monster monster = (Monster) fighter;
            return monster.getApcost();
        }
        return 0;
    }
}