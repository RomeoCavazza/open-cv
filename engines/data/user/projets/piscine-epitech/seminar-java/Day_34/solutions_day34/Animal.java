public class Animal {
    protected String name;
    protected int legs;
    protected Type type;
    
    protected enum Type {MAMMAL, FISH, BIRD};

    private static int numberOfAnimals = 0;
    private static int numberOfMammals = 0;
    private static int numberOfFish = 0;
    private static int numberOfBirds = 0;

    protected Animal(String name, int legs, Type type) {
        this.name = name;
        this.legs = legs;
        this.type = type;
        numberOfAnimals++;
        
        switch (type) {
            case MAMMAL: numberOfMammals++; break;
            case FISH: numberOfFish++; break;
            case BIRD: numberOfBirds++; break;
        }
        
        System.out.println("My name is " + name + " and I am a " + type.toString().toLowerCase() + "!");
    }

    public String getName() { return this.name; }
    public int getLegs() { return this.legs; }
    public String getType() { return this.type.toString().toLowerCase(); }

    public static int getNumberOfAnimals() {
        if (numberOfAnimals == 0) {
            System.out.println("There are currently 0 animals in our world.");
        } else if (numberOfAnimals == 1) {
            System.out.println("There is currently 1 animal in our world.");
        } else {
            System.out.println("There are currently " + numberOfAnimals + " animals in our world.");
        }
        return numberOfAnimals;
    }

    public static int getNumberOfMammals() {
        if (numberOfMammals == 0) {
            System.out.println("There are currently 0 mammals in our world.");
        } else if (numberOfMammals == 1) {
            System.out.println("There is currently 1 mammal in our world.");
        } else {
            System.out.println("There are currently " + numberOfMammals + " mammals in our world.");
        }
        return numberOfMammals;
    }

    public static int getNumberOfFish() {
        if (numberOfFish == 0) {
            System.out.println("There are currently 0 fish in our world.");
        } else if (numberOfFish == 1) {
            System.out.println("There is currently 1 fish in our world.");
        } else {
            System.out.println("There are currently " + numberOfFish + " fish in our world.");
        }
        return numberOfFish;
    }

    public static int getNumberOfBirds() {
        if (numberOfBirds == 0) {
            System.out.println("There are currently 0 birds in our world.");
        } else if (numberOfBirds == 1) {
            System.out.println("There is currently 1 bird in our world.");
        } else {
            System.out.println("There are currently " + numberOfBirds + " birds in our world.");
        }
        return numberOfBirds;
    }
}