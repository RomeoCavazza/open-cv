public class RadScorpion extends Monster {
    private static int idCounter = 0;

    public RadScorpion() {
        super("RadScorpion #" + (++idCounter), 80, 50);
        System.out.println(getName() + ": Crrr!");
    }

    public int getDamage() {
        return 25;
    }

    public int getApcost() {
        return 8;
    }

    public boolean attack(Fighter target) {
        if (closeTarget != target) {
            System.out.println(name + ": I'm too far away from " + target.getName() + ".");
            return false;
        }
        
        if (ap < apcost) {
            return false;
        }
        
        int damage = getDamage();
        if (target instanceof Monster) {
            damage = damage;
        } else if (target instanceof AssaultTerminator) {
            damage = damage;
        } else {
            damage *= 2;
        }
        
        System.out.println(name + " attacks " + target.getName() + ".");
        ap -= apcost;
        target.receiveDamage(damage);
        return true;
    }
}