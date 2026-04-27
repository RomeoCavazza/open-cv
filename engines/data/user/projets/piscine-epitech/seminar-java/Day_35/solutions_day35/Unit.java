public abstract class Unit implements Fighter {
    protected String name;
    protected int hp;
    protected int ap;
    protected Fighter closeTarget;

    protected Unit(String name, int hp, int ap) {
        this.name = name;
        this.hp = hp;
        this.ap = ap;
    }

    public String getName() {
        return name;
    }

    public int getHp() {
        return hp;
    }
    
    public int getAp() {
        return ap;
    }

    public Fighter getCloseTarget() {
        return closeTarget;
    }
   
    public void receiveDamage(int damage) {
        hp -= damage;
        if (hp <= 0) {
            hp = 0;
        }
    }

    public boolean moveCloseTo(Fighter target) {
        if (target == this) {
            return false;
        }
        if (closeTarget == target) {
            return false;
        }
        closeTarget = target;
        System.out.println(name + " is moving closer to " + target.getName() + ".");
        return true;
    }

    public void recoverAP() {
        ap += 7;
        if (ap > 50) {
            ap = 50;
        }
    }

    public abstract boolean equip(Weapon weapon);
    public abstract boolean attack(Fighter target);
}