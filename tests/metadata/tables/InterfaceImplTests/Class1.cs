using System;

namespace InterfaceImplTests
{
	interface ABC { }
	interface DEF { }

	public class Class1 : ABC, DEF, IDisposable
	{
		public void Dispose()
		{
			throw new NotImplementedException();
		}
	}
}
