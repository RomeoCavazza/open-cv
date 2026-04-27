public abstract class Character implements Movable, Comparable<Character> {
    protected String name;
    protected int life;
    protected int agility;
    protected int strength;
    protected int wit;
    protected final String RPGClass;
    protected int capacity;

    protected Character(String name, String RPGClass) {
        this.name = name;
        this.life = 50;
        this.agility = 2;
        this.strength = 2;
        this.wit = 2;
        this.RPGClass = RPGClass;
        this.capacity = 0;
    }

    protected Character(String name, String RPGClass, int capacity) {
        this.name = name;
        this.life = 50;
        this.agility = 2;
        this.strength = 2;
        this.wit = 2;
        this.RPGClass = RPGClass;
        this.capacity = capacity;
    }

    public String getName() {
        return name;
    }

    public int getLife() {
        return life;
    }

    public int getAgility() {
        return agility;
    }

    public int getStrength() {
        return strength;
    }

    public int getWit() {
        return wit;
    }

    public String getRPGClass() {
        return RPGClass;
    }

    public int getCapacity() {
        return capacity;
    }

    @Override
    public int compareTo(Character other) {
        if (other == null) {
            return 0;
        }
        
        // Si les deux personnages sont du même type (Warrior-Warrior ou Mage-Mage)
        if (this.getClass() == other.getClass()) {
            return Integer.compare(this.capacity, other.capacity);
        }
        
        // Si un Warrior et un Mage
        if (this instanceof Warrior && other instanceof Mage) {
            // Warrior est plus grand si sa capacité est un multiple de la capacité du Mage
            if (this.capacity != 0 && other.capacity != 0 && this.capacity % other.capacity == 0) {
                return 1;
            }
            // Sinon Mage est plus grand
            return -1;
        }
        
        if (this instanceof Mage && other instanceof Warrior) {
            // Warrior est plus grand si sa capacité est un multiple de la capacité du Mage
            if (other.capacity != 0 && this.capacity != 0 && other.capacity % this.capacity == 0) {
                return -1;
            }
            // Sinon Mage est plus grand
            return 1;
        }
        
        return 0;
    }

    public void attack(String weapon) {
        System.out.println(name + ": Rrrrrrrrr.... " + weapon);
    }

    public void moveRight() {
        System.out.println(name + ": moves right");
    }

    public void moveLeft() {
        System.out.println(name + ": moves left");
    }

    public void moveForward() {
        System.out.println(name + ": moves forward");
    }

    public void moveBack() {
        System.out.println(name + ": moves back");
    }

    public final void unsheathe() {
        System.out.println(name + ": unsheathes his weapon.");
    }
}