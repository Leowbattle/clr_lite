using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace EmptyExe
{
	public class Program : I
	{
		static void Main(string[] args)
		{
			Console.WriteLine("Hello World!");
		}

		public void H()
		{

		}
	}

	interface I
	{
		void H();
	}
}
