public interface Fighter {
    boolean equip(Weapon weapon);
    boolean attack(Fighter enemy);
    void receiveDamage(int damage);
    boolean moveCloseTo(Fighter enemy);
    void recoverAP();
    String getName();
    int getAp();
    int getHp();
}