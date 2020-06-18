using System;
using System.Runtime.InteropServices;

namespace MethodDefTests
{
	public abstract class Class1
	{
		public void Doit() { }
		static void Static() { }
		public virtual void Virtual() { }
		public abstract void Abstract();
		[DllImport("nice")]
		static extern void PInvoke();
	}
}
