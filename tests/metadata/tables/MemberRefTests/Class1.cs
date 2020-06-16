using System;

namespace MemberRefTests
{
	public class Class1
	{
		void Hello()
		{
			Console.WriteLine("Hello world");
			string s = "Hello";
			Console.WriteLine($"Length = {s.Length}");
		}
	}
}
