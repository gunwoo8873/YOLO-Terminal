using Terminal.Gui;

namespace Gui
{
  public class Layout
  {
    public static void Run()
    {
      // Init runtime
      Application.Init();

      // Return to window title and layout
      var win = View.Text.Title();
      var menu = View.Button.Menu();
      Application.Top.Add(win, menu);

      // UI load runtime
      Application.Run();
    }
  }

  // public class Element
  // {
  //   public static Window Title()
  //   {
  //     // var top = Application.Top;
  //     // Window size and main title feature
  //     var win = new Window("Test Terminal.Gui")
  //     {
  //       X = 0,
  //       Y = 0,
  //       Width = Dim.Fill(),
  //       Height = Dim.Fill(),
  //     };

  //     // Feature layout
  //     var label = new Label("Test Label")
  //     {
  //       X = Pos.Center(),
  //       Y = Pos.Center()
  //     };
  //     win.Add(label);

  //     return win;
  //   }
  // }
}

namespace View
{
  public class Text
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
          // @gunwoo8873 : Add, Remove, Move, etc... and create date timestamp checking for looking
          // new MenuItem("Add", "",)
          // new MenuItem("Remove", "",)
          // new MenuItem("Move", "",)
        }),
        new MenuBarItem("Options", new MenuItem[] {
          // @gunwoo8873 : Options action to color for background, font, etc... selected
          // new MenuItem("Color", "", () => Application.Add())
        }),
        // @gunwoo8873 : Exit action to new small pop window and Y/N selected
        new MenuBarItem("_Exit", "", () => Application.RequestStop()),
      });

      return menu;
    }
  }
}


public class RunTime
{
  public static void Main(string[] args)
  {
    Gui.Layout.Run();
  }
}
