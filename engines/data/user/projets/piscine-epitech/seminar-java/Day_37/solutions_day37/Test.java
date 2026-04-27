import java.util.List;
import java.util.ArrayList;

public class Test {
    public static void main(String[] args) {
        System.out.println("=== Test Exercise 1: Solo ===");
        Solo<String> strSolo = new Solo<>("toto");
        System.out.println("Value: " + strSolo.getValue());
        strSolo.setValue("tata");
        System.out.println("New value: " + strSolo.getValue());
        
        Solo<Integer> intSolo = new Solo<>(42);
        System.out.println("Integer value: " + intSolo.getValue());
        intSolo.setValue(1337);
        System.out.println("New integer value: " + intSolo.getValue());
        System.out.println();

        System.out.println("=== Test Exercise 2: Pair ===");
        Pair<String, Integer> pair = new Pair<>("hello", 42);
        pair.display();
        System.out.println("First: " + pair.getFirst());
        System.out.println("Second: " + pair.getSecond());
        System.out.println();

        System.out.println("=== Test Exercise 3: Duet ===");
        String minStr = Duet.min("apple", "zoo");
        String maxStr = Duet.max("apple", "zoo");
        System.out.println("Min string: " + minStr);
        System.out.println("Max string: " + maxStr);
        
        Integer minInt = Duet.min(10, 25);
        Integer maxInt = Duet.max(10, 25);
        System.out.println("Min integer: " + minInt);
        System.out.println("Max integer: " + maxInt);
        System.out.println();

        System.out.println("=== Test Exercise 4: Battalion ===");
        List<Mage> mages = new ArrayList<>();
        mages.add(new Mage("Merlin"));
        mages.add(new Mage("Mandrake"));
        List<Warrior> warriors = new ArrayList<>();
        warriors.add(new Warrior("Spartacus"));
        warriors.add(new Warrior("Clovis"));
        
        Battalion battalion = new Battalion();
        battalion.add(mages);
        battalion.add(warriors);
        System.out.println("Display battalion:");
        battalion.display();
        System.out.println();

        System.out.println("=== Test Exercise 5 & 6: Capacity and Fight ===");
        Character merlin = new Mage("Merlin", 12);
        Character gandalf = new Mage("Gandalf", 12);
        Character mandrake = new Mage("Mandrake", 9);
        Character achilles = new Warrior("Achilles", 240);
        
        System.out.println("merlin.compareTo(mandrake): " + merlin.compareTo(mandrake));
        System.out.println("merlin.compareTo(achilles): " + merlin.compareTo(achilles));
        System.out.println("gandalf.compareTo(merlin): " + gandalf.compareTo(merlin));
        System.out.println();
        
        List<Character> fighters = new ArrayList<>();
        fighters.add(merlin);
        fighters.add(achilles);
        Battalion battleBattalion = new Battalion();
        battleBattalion.add(fighters);
        System.out.println("Before fight:");
        battleBattalion.display();
        System.out.println("Fight result: " + battleBattalion.fight());
        System.out.println("After fight:");
        battleBattalion.display();
    }
}
