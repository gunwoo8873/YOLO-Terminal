using Terminal.Gui;

namespace Screen
{
  public class Layout
  {
    public static void Run()
    {
      // Init runtime
      Application.Init();

      // Return to window title and layout
      var win = Headers.Title.Title();
      var menu = Headers.Button.Menu();
      Application.Top.Add(win, menu);

      // UI load runtime
      Application.Run();
    }
  }
}

namespace Headers
{
  public class Title
  {
    public static Window Title()
    {
      // Window size and main title feature
      var win = new Window("Test Terminal.Gui")
      {
        X = 0,
        Y = 0,
        Width = Dim.Fill(),
        Height = Dim.Fill(),
      };

      // var label = new Label("Test Label")
      // {
      //   X = Pos.Center(),
      //   Y = Pos.Center()
      // };
      // win.Add(label);

      return win;
    }
  }

  public class Button
  {
    public static MenuBar Menu()
    {
      var menu = new MenuBar(new MenuBarItem[] {
        new MenuBarItem("File", new MenuItem[] {
          new MenuItem("Add", "", () => {}),
          new MenuItem("Remove", "", () => {}),
          new MenuItem("Move", "", () => {}),
        }),
        new MenuBarItem("Options", new MenuItem[] {
          new MenuItem("Color", "", () => {}),
        }),
        new MenuBarItem("Exit", "", () => Application.RequestStop()),
      });

      return menu;
    }
  }
}

public class RunTime
{
  public static void Main(string[] args)
  {
    Screen.Layout.Run();
  }
}