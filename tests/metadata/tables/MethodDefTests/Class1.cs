using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace MethodDefTests
{
	public class Class1
	{
		void Method() { }
		static void Static() { }
		void Vararg(__arglist) { }
		void Generic<T>(T t, int x = 1) { }
		int ReturnsInt() => 42;
		Class1 ReturnsClass1() => new Class1();
	}
}
