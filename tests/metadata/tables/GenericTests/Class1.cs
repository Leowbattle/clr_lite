using System;

namespace GenericTests
{
	public class Class1
	{
		T MakeIt<T>() where T : new() => new T();
	}
}
