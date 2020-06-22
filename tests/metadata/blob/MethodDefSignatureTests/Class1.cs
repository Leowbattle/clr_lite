using System;

namespace MethodDefSignatureTests
{
	public class Class1
	{
		void Basic() { }
		static void Static() { }
		void Generic<T>(T t) { }
		int ReturnsInt() => 42;
		int Add(int a, int b) => a + b;
	}
}
