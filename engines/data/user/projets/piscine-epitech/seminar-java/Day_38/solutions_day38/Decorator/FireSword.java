package Decorator;

public class FireSword extends StuffDecorator {
    public FireSword(Warrior holder) {
        super(holder);
        System.out.println("I can slice and burn like the wind and the flames.");
    }
    
    @Override
    public int getDmg() {
        return holder.getDmg() + 3;
    }
}
