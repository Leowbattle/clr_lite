using System;

namespace PropertyTests
{
	public class Class1
	{
		int Property { get; }
	}

	class Class2
	{

	}

	class Class3
	{
		float Prop2 { get; set; }
		double Prop3 => 1;
	}
}
