using System;

namespace EventTests
{
	public class Class1
	{
		event Action Hello;
		event Action Hello2;
	}

	class Class2
	{
		event Action Hi3;
		event Action Hi4;
	}

	class Class3
	{
		event Action AAA;
	}
}
