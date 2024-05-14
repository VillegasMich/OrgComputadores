import kareltherobot.*;
import java.awt.Color;
import java.util.Random;

public class KarelParalelismo implements Directions {
  public static void main(String[] args) {
    /* Variables */
    Random rand = new Random();
    int r = 0;
    int n = 0;
    boolean e = false;

    /* Arguments */
    try {
      if (args.length > 0) {
        if (args[0].equals("-r")) {
          r = Integer.parseInt(args[1]);
          if (r != 1 && r != 2 && r != 4) {
            throw new Exception("Argument -r need to be 1, 2 or 4");
          }
        }
        if (args.length >= 3 && args[2].equals("-e")) {
          e = true;
        }
      } else {
        throw new Exception("Argument -r not found");
      }
    } catch (Exception exe) {
      System.err.println(exe);
      System.exit(0);
    }

    /* World Setup */
    setUpWprld(r, rand);

    /* Robots */
    Thread[] threadsArr = setUpRobots(r, n, e, rand);
    for (Thread robot : threadsArr) {
      robot.start();
    }
  }

  static void setUpWprld(int r, Random rand) {
    World.setSize(8, 10); // 8 calles 10 avenidas
    for (int i = 0; i < r * 100; i++) { // i < r*100
      int randStreet = rand.nextInt(3, 9);
      int randAvenue = rand.nextInt(1, 11);
      World.placeBeepers(randStreet, randAvenue, 1);
    }
    World.setVisible(true);
  }

  static Thread[] setUpRobots(int r, int n, boolean e, Random rand) {
    if (e) {
      while (true) {
        int tmp = rand.nextInt(1, 9);
        if (tmp % 2 == 0) {
          n = tmp;
          break;
        }
      }
    }
    Thread[] threadsArr = new Thread[r];
    for (int i = 0; i < r; i++) {
      if (e) {
        ParalelRobot robot = new ParalelRobot(i + 1, 2, East, 0, new Color((int) (Math.random() * 0x1000000)), n);
        Thread robotThread = new Thread(robot);
        threadsArr[i] = robotThread;
      } else {
        while (true) {
          int tmp = rand.nextInt(1, 9);
          if (tmp % 2 == 0) {
            ParalelRobot robot = new ParalelRobot(i + 1, 2, East, 0, new Color((int) (Math.random() * 0x1000000)), tmp);
            Thread robotThread = new Thread(robot);
            threadsArr[i] = robotThread;
            break;
          }
        }
      }
    }
    return threadsArr;
  }
}

class ParalelRobot extends Robot implements Runnable {
  public int limitBeepers;

  public ParalelRobot(int Street, int Avenue, Direction direction, int beepers, Color color, int limitBeepers) {
    super(Street, Avenue, direction, beepers, color);
    this.limitBeepers = limitBeepers;
    World.setupThread(this);
  }

  public void work() {
    while (true) {
      while (this.frontIsClear()) {
        this.move();
        // System.out.println()
        if (this.nextToABeeper()) {
          this.pickBeeper();
          this.turnLeft();
        }
      }
      this.turnLeft();
    }
    // turnOff();
  }

  void turnRight() {
    turnLeft();
    turnLeft();
    turnLeft();
  }

  @Override
  public void run() {
    work();
  }
}
