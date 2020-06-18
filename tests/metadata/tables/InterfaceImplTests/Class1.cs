using System;

namespace InterfaceImplTests
{
	interface ABC { }
	interface DEF { }

	public class Class1 : ABC
	{
	}

	class Class2 : DEF, IDisposable
	{
		public void Dispose()
		{
			throw new NotImplementedException();
		}
	}
}
