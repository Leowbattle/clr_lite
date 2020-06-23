using System;

namespace PropertySignatureTests
{
	public class Class1
	{
		int Prop { get; }
		string Prop2 => "Hello there!";
		int this[int x, int y] => 42;
	}
}
