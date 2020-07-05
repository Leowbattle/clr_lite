using System.Reflection;

namespace System.Runtime.CompilerServices
{
    public enum MethodCodeType
    {
        IL = MethodImplAttributes.IL,
        Native = MethodImplAttributes.Native,
        OPTIL = MethodImplAttributes.OPTIL,
        Runtime = MethodImplAttributes.Runtime
    }
}