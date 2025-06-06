using Terminal.Gui;

namespace Terminal
{
  public class RunTime
  {
    public static void Run()
    {
      // Init runtime
      Application.Init();
      
      // Return to window title and layout
      var win = Element.Title();
      Application.Top.Add(win);

      // UI load runtime
      Application.Run();
    }
  }

  public class Element
  {
    public static Window Title()
    {
      // var top = Application.Top;
      // Window size and main title feature
      var win = new Window("Test Terminal.Gui")
      {
        X = 0,
        Y = 0,
        Width = Dim.Fill(),
        Height = Dim.Fill(),
      };

      // Feature layout
      var label = new Label("Test Label")
      {
        X = Pos.Center(),
        Y = Pos.Center()
      };
      win.Add(label);

      return win;
    }
  }
}

public class Compiler
{
  public static void Main(string[] args)
  {
    Terminal.RunTime.Run();
  }
}
