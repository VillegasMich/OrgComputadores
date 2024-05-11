import kareltherobot.*;
import java.awt.Color;

public class MiPrimerRobot implements Directions {
  public static void main(String[] args) {

    World.readWorld("Cage.kwld");
    World.setVisible(true);

    ParalelRobot first = new ParalelRobot(1, 1, East, 0, Color.RED);
    ParalelRobot second = new ParalelRobot(4, 1, East, 0, Color.BLUE);
    Thread firstRobot = new Thread(first);
    Thread secondRobot = new Thread(second);
    firstRobot.start();
    secondRobot.start();
  }
}

class ParalelRobot extends Robot implements Runnable {
  public ParalelRobot(int Street, int Avenue, Direction direction, int beepers, Color color) {
    super(Street, Avenue, direction, beepers, color);
    World.setupThread(this);
  }

  public void work() {
    while (true) {
      while (frontIsClear()) {
        move();
        if (nextToABeeper()) {
          pickBeeper();
          turnLeft();
        }
      }
      turnLeft();
    }
    // turnOff();
  }

  void turnRight() {
    turnLeft();
    turnLeft();
    turnLeft();
  }

  public void run() {
    work();
  }
}
