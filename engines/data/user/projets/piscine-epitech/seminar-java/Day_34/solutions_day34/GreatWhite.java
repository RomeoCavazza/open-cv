public class GreatWhite extends Shark {
    public GreatWhite(String name) {
        super(name);
    }
    
    @Override
    public boolean canEat(Animal animal) {
        if (animal == this) {
            return false;
        }
        if (animal instanceof Canary) {
            return false;
        }
        return !(animal instanceof Shark);
    }
    
    @Override
    public void eat(Animal animal) {
        if (animal instanceof Canary) {
            System.out.println(name + ": Next time you try to give me that to eat, I'll eat you instead.");
            return;
        }
        
        if (canEat(animal)) {
            System.out.println(name + " ate a " + animal.getType() + " named " + animal.getName() + ".");
            this.frenzy = false;
            
            if (animal instanceof Shark) {
                System.out.println(name + ": The best meal one could wish for.");
            }
        } else {
            System.out.println(name + ": It's not worth my time.");
        }
    }
}
