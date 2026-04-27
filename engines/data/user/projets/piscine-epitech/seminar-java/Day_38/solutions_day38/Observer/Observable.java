package Observer;

public interface Observable {
    void addObserver(Observer observer);
    boolean notifyObservers();
}
