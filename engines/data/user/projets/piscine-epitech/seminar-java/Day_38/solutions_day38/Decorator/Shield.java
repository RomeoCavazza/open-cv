package Decorator;

public class Shield extends StuffDecorator {
    public Shield(Warrior holder) {
        super(holder);
        System.out.println("May this shield protect me against every enemy.");
    }
    
    @Override
    public int getHp() {
        return holder.getHp() + 10;
    }
}

