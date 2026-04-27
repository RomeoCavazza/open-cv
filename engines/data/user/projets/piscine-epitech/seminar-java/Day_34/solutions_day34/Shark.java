public class Shark extends Animal {
    protected boolean frenzy;

    public Shark(String name) {
        super(name, 0, Type.FISH);
        this.frenzy = false;
        System.out.println("A KILLER IS BORN!");
    }

    public void smellBlood(boolean frenzy) {
        this.frenzy = frenzy;
    }

    public void status() {
        if (frenzy == true) {
            System.out.println(name + " is smelling blood and wants to kill.");
        }
        else {
            System.out.println(name + " is swimming peacefully.");
        }
    }
    
    public boolean canEat(Animal animal) {
        if (animal == this) {
            return false;
        }
        return true;
    }
    
    public void eat(Animal animal) {
        if (canEat(animal)) {
            System.out.println(name + " ate a " + animal.getType() + " named " + animal.getName() + ".");
            this.frenzy = false;
        } else {
            System.out.println(name + ": It's not worth my time.");
        }
    }
}