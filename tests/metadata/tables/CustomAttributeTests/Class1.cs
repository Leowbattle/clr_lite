using System;

namespace CustomAttributeTests
{
	[AttributeUsage(AttributeTargets.Class)]
	class MyAttribute : Attribute
	{

	}
	
	[MyAttribute]
	public class Class1
	{
	}
}
