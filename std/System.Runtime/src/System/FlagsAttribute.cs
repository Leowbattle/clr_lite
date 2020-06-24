using System.Reflection;

namespace System
{
	[AttributeUsage(AttributeTargets.Enum, Inherited = false)]
	public class FlagsAttribute : Attribute
	{
		public FlagsAttribute()
		{
		}
	}
}
