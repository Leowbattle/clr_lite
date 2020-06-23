using System;

namespace MethodSignatureTests
{
	public class Class1
	{
		void Basic() { }
		static void Static() { }
		void Vararg(int x, __arglist) { }
		void Generic<T>(T t) { }
		int ReturnsInt() => 42;
		int Add(int a, int b) => a + b;

		void Doit()
		{
			Vararg(0, __arglist(1, 2, 3));
		}
	}
}
