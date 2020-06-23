using System;
using System.Collections.Generic;
using System.Linq;

namespace GenericInstanceTests
{
	public class Class1
	{
		List<int> a = new List<int>();
		Dictionary<string, List<string>> b = new Dictionary<string, List<string>>();

		void Generic<T, U>() { }

		void Doit()
		{
			var x = a.Where(x => x > 10).Sum();
			Generic<string, object>();
		}
	}
}
