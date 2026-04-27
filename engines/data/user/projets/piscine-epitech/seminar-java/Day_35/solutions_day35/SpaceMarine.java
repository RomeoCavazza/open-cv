public abstract class SpaceMarine extends Unit {
    protected Weapon weapon;

    public SpaceMarine(String name, int hp, int ap) {
        super(name, hp, ap);
        this.weapon = null;
    }

    public Weapon getWeapon() {
        return weapon;
    }

    public boolean equip(Weapon weapon) {
        if (weapon.getOwner() != null && weapon.getOwner() != this) {
            return false;
        }
        this.weapon = weapon;
        weapon.setOwner(this);
        System.out.println(name + " has been equipped with a " + weapon.getName() + ".");
        return true;
    }

    public boolean attack(Fighter target) {
        if (weapon == null) {
            System.out.println(name + ": Hey, this is crazy. I'm not going to fight this empty-handed.");
            return false;
        }

        if (weapon.isMelee() && closeTarget != target) {
            System.out.println(name + ": I'm too far away from " + target.getName() + ".");
            return false;
        }

        if (ap < weapon.getApcost()) {
            return false;
        }

        System.out.println(name + " attacks " + target.getName() + " with a " + weapon.getName() + ".");
        weapon.attack();
        target.receiveDamage(weapon.getDamage());
        ap -= weapon.getApcost();
        return true;
    }

    public boolean moveCloseTo(Fighter target) {
        if (weapon != null && weapon.isMelee()) {
            return super.moveCloseTo(target);
        }
        return false;
    }

    public void recoverAP() {
        ap += 9;
        if (ap > 50) {
            ap = 50;
        }
    }
}

