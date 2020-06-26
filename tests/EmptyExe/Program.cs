using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace EmptyExe
{
	public class Program
	{
		static void Main(string[] args)
		{
			Console.WriteLine("Hello World!");
		}

		[MethodImpl(MethodImplOptions.InternalCall)]
		static extern void H();
	}

	//interface I
	//{
	//	void H();
	//}
}
