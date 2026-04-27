public abstract class Monster extends Unit {
    protected int damage;
    protected int apcost;

    public Monster(String name, int hp, int ap) {
        super(name, hp, ap);
        this.damage = 0;
        this.apcost = 0;
    }

    public int getDamage() {
        return damage;
    }

    public int getApcost() {
        return apcost;
    }

    public boolean equip(Weapon weapon) {
        System.out.println("Monsters are proud and fight with their own bodies.");
        return false;
    }

    public boolean attack(Fighter target) {
        if (closeTarget != target) {
            System.out.println(name + ": I'm too far away from " + target.getName() + ".");
            return false;
        }
        
        if (ap < apcost) {
            return false;
        }
        
        System.out.println(name + " attacks " + target.getName() + ".");
        ap -= apcost;
        target.receiveDamage(damage);
        return true;
    }
}