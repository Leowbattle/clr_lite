using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

namespace InterpreterTests
{
	/// <summary>
	/// This class contains test cases for supported CIL instructions in ClrLite.
	/// </summary>
	class Tests
	{
		static void Empty() { }

		void NonStatic() { }

		static int Fibonacci(int n)
		{
			if (n < 2)
			{
				return n;
			}
			return Fibonacci(n - 1) + Fibonacci(n - 2);
		}

		#region Number loading
		static int Ldc_I4_M1()
		{
			return -1;
		}

		static int Ldc_I4_4()
		{
			return 4;
		}

		static int Ldc_I4_S()
		{
			return 100;
		}

		static int Ldc_I4()
		{
			return 1000;
		}

		static float Ldc_R4()
		{
			return 3.14159f;
		}

		static double Ldc_R8()
		{
			return 1.2345678;
		}
		#endregion

		#region Local variables
		// There need to be 256 locals to use the Ldloc and Stloc instructions, hence this monstrosity
		static int Locals()
		{
			int local_0 = 42;
			int local_1 = local_0;
			int local_2 = local_1;
			int local_3 = local_2;
			int local_4 = local_3;
			int local_5 = local_4;
			int local_6 = local_5;
			int local_7 = local_6;
			int local_8 = local_7;
			int local_9 = local_8;
			int local_10 = local_9;
			int local_11 = local_10;
			int local_12 = local_11;
			int local_13 = local_12;
			int local_14 = local_13;
			int local_15 = local_14;
			int local_16 = local_15;
			int local_17 = local_16;
			int local_18 = local_17;
			int local_19 = local_18;
			int local_20 = local_19;
			int local_21 = local_20;
			int local_22 = local_21;
			int local_23 = local_22;
			int local_24 = local_23;
			int local_25 = local_24;
			int local_26 = local_25;
			int local_27 = local_26;
			int local_28 = local_27;
			int local_29 = local_28;
			int local_30 = local_29;
			int local_31 = local_30;
			int local_32 = local_31;
			int local_33 = local_32;
			int local_34 = local_33;
			int local_35 = local_34;
			int local_36 = local_35;
			int local_37 = local_36;
			int local_38 = local_37;
			int local_39 = local_38;
			int local_40 = local_39;
			int local_41 = local_40;
			int local_42 = local_41;
			int local_43 = local_42;
			int local_44 = local_43;
			int local_45 = local_44;
			int local_46 = local_45;
			int local_47 = local_46;
			int local_48 = local_47;
			int local_49 = local_48;
			int local_50 = local_49;
			int local_51 = local_50;
			int local_52 = local_51;
			int local_53 = local_52;
			int local_54 = local_53;
			int local_55 = local_54;
			int local_56 = local_55;
			int local_57 = local_56;
			int local_58 = local_57;
			int local_59 = local_58;
			int local_60 = local_59;
			int local_61 = local_60;
			int local_62 = local_61;
			int local_63 = local_62;
			int local_64 = local_63;
			int local_65 = local_64;
			int local_66 = local_65;
			int local_67 = local_66;
			int local_68 = local_67;
			int local_69 = local_68;
			int local_70 = local_69;
			int local_71 = local_70;
			int local_72 = local_71;
			int local_73 = local_72;
			int local_74 = local_73;
			int local_75 = local_74;
			int local_76 = local_75;
			int local_77 = local_76;
			int local_78 = local_77;
			int local_79 = local_78;
			int local_80 = local_79;
			int local_81 = local_80;
			int local_82 = local_81;
			int local_83 = local_82;
			int local_84 = local_83;
			int local_85 = local_84;
			int local_86 = local_85;
			int local_87 = local_86;
			int local_88 = local_87;
			int local_89 = local_88;
			int local_90 = local_89;
			int local_91 = local_90;
			int local_92 = local_91;
			int local_93 = local_92;
			int local_94 = local_93;
			int local_95 = local_94;
			int local_96 = local_95;
			int local_97 = local_96;
			int local_98 = local_97;
			int local_99 = local_98;
			int local_100 = local_99;
			int local_101 = local_100;
			int local_102 = local_101;
			int local_103 = local_102;
			int local_104 = local_103;
			int local_105 = local_104;
			int local_106 = local_105;
			int local_107 = local_106;
			int local_108 = local_107;
			int local_109 = local_108;
			int local_110 = local_109;
			int local_111 = local_110;
			int local_112 = local_111;
			int local_113 = local_112;
			int local_114 = local_113;
			int local_115 = local_114;
			int local_116 = local_115;
			int local_117 = local_116;
			int local_118 = local_117;
			int local_119 = local_118;
			int local_120 = local_119;
			int local_121 = local_120;
			int local_122 = local_121;
			int local_123 = local_122;
			int local_124 = local_123;
			int local_125 = local_124;
			int local_126 = local_125;
			int local_127 = local_126;
			int local_128 = local_127;
			int local_129 = local_128;
			int local_130 = local_129;
			int local_131 = local_130;
			int local_132 = local_131;
			int local_133 = local_132;
			int local_134 = local_133;
			int local_135 = local_134;
			int local_136 = local_135;
			int local_137 = local_136;
			int local_138 = local_137;
			int local_139 = local_138;
			int local_140 = local_139;
			int local_141 = local_140;
			int local_142 = local_141;
			int local_143 = local_142;
			int local_144 = local_143;
			int local_145 = local_144;
			int local_146 = local_145;
			int local_147 = local_146;
			int local_148 = local_147;
			int local_149 = local_148;
			int local_150 = local_149;
			int local_151 = local_150;
			int local_152 = local_151;
			int local_153 = local_152;
			int local_154 = local_153;
			int local_155 = local_154;
			int local_156 = local_155;
			int local_157 = local_156;
			int local_158 = local_157;
			int local_159 = local_158;
			int local_160 = local_159;
			int local_161 = local_160;
			int local_162 = local_161;
			int local_163 = local_162;
			int local_164 = local_163;
			int local_165 = local_164;
			int local_166 = local_165;
			int local_167 = local_166;
			int local_168 = local_167;
			int local_169 = local_168;
			int local_170 = local_169;
			int local_171 = local_170;
			int local_172 = local_171;
			int local_173 = local_172;
			int local_174 = local_173;
			int local_175 = local_174;
			int local_176 = local_175;
			int local_177 = local_176;
			int local_178 = local_177;
			int local_179 = local_178;
			int local_180 = local_179;
			int local_181 = local_180;
			int local_182 = local_181;
			int local_183 = local_182;
			int local_184 = local_183;
			int local_185 = local_184;
			int local_186 = local_185;
			int local_187 = local_186;
			int local_188 = local_187;
			int local_189 = local_188;
			int local_190 = local_189;
			int local_191 = local_190;
			int local_192 = local_191;
			int local_193 = local_192;
			int local_194 = local_193;
			int local_195 = local_194;
			int local_196 = local_195;
			int local_197 = local_196;
			int local_198 = local_197;
			int local_199 = local_198;
			int local_200 = local_199;
			int local_201 = local_200;
			int local_202 = local_201;
			int local_203 = local_202;
			int local_204 = local_203;
			int local_205 = local_204;
			int local_206 = local_205;
			int local_207 = local_206;
			int local_208 = local_207;
			int local_209 = local_208;
			int local_210 = local_209;
			int local_211 = local_210;
			int local_212 = local_211;
			int local_213 = local_212;
			int local_214 = local_213;
			int local_215 = local_214;
			int local_216 = local_215;
			int local_217 = local_216;
			int local_218 = local_217;
			int local_219 = local_218;
			int local_220 = local_219;
			int local_221 = local_220;
			int local_222 = local_221;
			int local_223 = local_222;
			int local_224 = local_223;
			int local_225 = local_224;
			int local_226 = local_225;
			int local_227 = local_226;
			int local_228 = local_227;
			int local_229 = local_228;
			int local_230 = local_229;
			int local_231 = local_230;
			int local_232 = local_231;
			int local_233 = local_232;
			int local_234 = local_233;
			int local_235 = local_234;
			int local_236 = local_235;
			int local_237 = local_236;
			int local_238 = local_237;
			int local_239 = local_238;
			int local_240 = local_239;
			int local_241 = local_240;
			int local_242 = local_241;
			int local_243 = local_242;
			int local_244 = local_243;
			int local_245 = local_244;
			int local_246 = local_245;
			int local_247 = local_246;
			int local_248 = local_247;
			int local_249 = local_248;
			int local_250 = local_249;
			int local_251 = local_250;
			int local_252 = local_251;
			int local_253 = local_252;
			int local_254 = local_253;
			int local_255 = local_254;
			return local_255;
		}
		#endregion

		#region Parameters
		// There need to be 256 parameters to use the Ldarg and Starg instructions, hence this monstrosity
		static int Parameters(int arg_0, int arg_1, int arg_2, int arg_3, int arg_4, int arg_5, int arg_6, int arg_7, int arg_8, int arg_9, int arg_10, int arg_11, int arg_12, int arg_13, int arg_14, int arg_15, int arg_16, int arg_17, int arg_18, int arg_19, int arg_20, int arg_21, int arg_22, int arg_23, int arg_24, int arg_25, int arg_26, int arg_27, int arg_28, int arg_29, int arg_30, int arg_31, int arg_32, int arg_33, int arg_34, int arg_35, int arg_36, int arg_37, int arg_38, int arg_39, int arg_40, int arg_41, int arg_42, int arg_43, int arg_44, int arg_45, int arg_46, int arg_47, int arg_48, int arg_49, int arg_50, int arg_51, int arg_52, int arg_53, int arg_54, int arg_55, int arg_56, int arg_57, int arg_58, int arg_59, int arg_60, int arg_61, int arg_62, int arg_63, int arg_64, int arg_65, int arg_66, int arg_67, int arg_68, int arg_69, int arg_70, int arg_71, int arg_72, int arg_73, int arg_74, int arg_75, int arg_76, int arg_77, int arg_78, int arg_79, int arg_80, int arg_81, int arg_82, int arg_83, int arg_84, int arg_85, int arg_86, int arg_87, int arg_88, int arg_89, int arg_90, int arg_91, int arg_92, int arg_93, int arg_94, int arg_95, int arg_96, int arg_97, int arg_98, int arg_99, int arg_100, int arg_101, int arg_102, int arg_103, int arg_104, int arg_105, int arg_106, int arg_107, int arg_108, int arg_109, int arg_110, int arg_111, int arg_112, int arg_113, int arg_114, int arg_115, int arg_116, int arg_117, int arg_118, int arg_119, int arg_120, int arg_121, int arg_122, int arg_123, int arg_124, int arg_125, int arg_126, int arg_127, int arg_128, int arg_129, int arg_130, int arg_131, int arg_132, int arg_133, int arg_134, int arg_135, int arg_136, int arg_137, int arg_138, int arg_139, int arg_140, int arg_141, int arg_142, int arg_143, int arg_144, int arg_145, int arg_146, int arg_147, int arg_148, int arg_149, int arg_150, int arg_151, int arg_152, int arg_153, int arg_154, int arg_155, int arg_156, int arg_157, int arg_158, int arg_159, int arg_160, int arg_161, int arg_162, int arg_163, int arg_164, int arg_165, int arg_166, int arg_167, int arg_168, int arg_169, int arg_170, int arg_171, int arg_172, int arg_173, int arg_174, int arg_175, int arg_176, int arg_177, int arg_178, int arg_179, int arg_180, int arg_181, int arg_182, int arg_183, int arg_184, int arg_185, int arg_186, int arg_187, int arg_188, int arg_189, int arg_190, int arg_191, int arg_192, int arg_193, int arg_194, int arg_195, int arg_196, int arg_197, int arg_198, int arg_199, int arg_200, int arg_201, int arg_202, int arg_203, int arg_204, int arg_205, int arg_206, int arg_207, int arg_208, int arg_209, int arg_210, int arg_211, int arg_212, int arg_213, int arg_214, int arg_215, int arg_216, int arg_217, int arg_218, int arg_219, int arg_220, int arg_221, int arg_222, int arg_223, int arg_224, int arg_225, int arg_226, int arg_227, int arg_228, int arg_229, int arg_230, int arg_231, int arg_232, int arg_233, int arg_234, int arg_235, int arg_236, int arg_237, int arg_238, int arg_239, int arg_240, int arg_241, int arg_242, int arg_243, int arg_244, int arg_245, int arg_246, int arg_247, int arg_248, int arg_249, int arg_250, int arg_251, int arg_252, int arg_253, int arg_254, int arg_255)
		{
			arg_0 = 10;
			arg_1 = arg_0;
			arg_2 = arg_1;
			arg_3 = arg_2;
			arg_4 = arg_3;
			arg_5 = arg_4;
			arg_6 = arg_5;
			arg_7 = arg_6;
			arg_8 = arg_7;
			arg_9 = arg_8;
			arg_10 = arg_9;
			arg_11 = arg_10;
			arg_12 = arg_11;
			arg_13 = arg_12;
			arg_14 = arg_13;
			arg_15 = arg_14;
			arg_16 = arg_15;
			arg_17 = arg_16;
			arg_18 = arg_17;
			arg_19 = arg_18;
			arg_20 = arg_19;
			arg_21 = arg_20;
			arg_22 = arg_21;
			arg_23 = arg_22;
			arg_24 = arg_23;
			arg_25 = arg_24;
			arg_26 = arg_25;
			arg_27 = arg_26;
			arg_28 = arg_27;
			arg_29 = arg_28;
			arg_30 = arg_29;
			arg_31 = arg_30;
			arg_32 = arg_31;
			arg_33 = arg_32;
			arg_34 = arg_33;
			arg_35 = arg_34;
			arg_36 = arg_35;
			arg_37 = arg_36;
			arg_38 = arg_37;
			arg_39 = arg_38;
			arg_40 = arg_39;
			arg_41 = arg_40;
			arg_42 = arg_41;
			arg_43 = arg_42;
			arg_44 = arg_43;
			arg_45 = arg_44;
			arg_46 = arg_45;
			arg_47 = arg_46;
			arg_48 = arg_47;
			arg_49 = arg_48;
			arg_50 = arg_49;
			arg_51 = arg_50;
			arg_52 = arg_51;
			arg_53 = arg_52;
			arg_54 = arg_53;
			arg_55 = arg_54;
			arg_56 = arg_55;
			arg_57 = arg_56;
			arg_58 = arg_57;
			arg_59 = arg_58;
			arg_60 = arg_59;
			arg_61 = arg_60;
			arg_62 = arg_61;
			arg_63 = arg_62;
			arg_64 = arg_63;
			arg_65 = arg_64;
			arg_66 = arg_65;
			arg_67 = arg_66;
			arg_68 = arg_67;
			arg_69 = arg_68;
			arg_70 = arg_69;
			arg_71 = arg_70;
			arg_72 = arg_71;
			arg_73 = arg_72;
			arg_74 = arg_73;
			arg_75 = arg_74;
			arg_76 = arg_75;
			arg_77 = arg_76;
			arg_78 = arg_77;
			arg_79 = arg_78;
			arg_80 = arg_79;
			arg_81 = arg_80;
			arg_82 = arg_81;
			arg_83 = arg_82;
			arg_84 = arg_83;
			arg_85 = arg_84;
			arg_86 = arg_85;
			arg_87 = arg_86;
			arg_88 = arg_87;
			arg_89 = arg_88;
			arg_90 = arg_89;
			arg_91 = arg_90;
			arg_92 = arg_91;
			arg_93 = arg_92;
			arg_94 = arg_93;
			arg_95 = arg_94;
			arg_96 = arg_95;
			arg_97 = arg_96;
			arg_98 = arg_97;
			arg_99 = arg_98;
			arg_100 = arg_99;
			arg_101 = arg_100;
			arg_102 = arg_101;
			arg_103 = arg_102;
			arg_104 = arg_103;
			arg_105 = arg_104;
			arg_106 = arg_105;
			arg_107 = arg_106;
			arg_108 = arg_107;
			arg_109 = arg_108;
			arg_110 = arg_109;
			arg_111 = arg_110;
			arg_112 = arg_111;
			arg_113 = arg_112;
			arg_114 = arg_113;
			arg_115 = arg_114;
			arg_116 = arg_115;
			arg_117 = arg_116;
			arg_118 = arg_117;
			arg_119 = arg_118;
			arg_120 = arg_119;
			arg_121 = arg_120;
			arg_122 = arg_121;
			arg_123 = arg_122;
			arg_124 = arg_123;
			arg_125 = arg_124;
			arg_126 = arg_125;
			arg_127 = arg_126;
			arg_128 = arg_127;
			arg_129 = arg_128;
			arg_130 = arg_129;
			arg_131 = arg_130;
			arg_132 = arg_131;
			arg_133 = arg_132;
			arg_134 = arg_133;
			arg_135 = arg_134;
			arg_136 = arg_135;
			arg_137 = arg_136;
			arg_138 = arg_137;
			arg_139 = arg_138;
			arg_140 = arg_139;
			arg_141 = arg_140;
			arg_142 = arg_141;
			arg_143 = arg_142;
			arg_144 = arg_143;
			arg_145 = arg_144;
			arg_146 = arg_145;
			arg_147 = arg_146;
			arg_148 = arg_147;
			arg_149 = arg_148;
			arg_150 = arg_149;
			arg_151 = arg_150;
			arg_152 = arg_151;
			arg_153 = arg_152;
			arg_154 = arg_153;
			arg_155 = arg_154;
			arg_156 = arg_155;
			arg_157 = arg_156;
			arg_158 = arg_157;
			arg_159 = arg_158;
			arg_160 = arg_159;
			arg_161 = arg_160;
			arg_162 = arg_161;
			arg_163 = arg_162;
			arg_164 = arg_163;
			arg_165 = arg_164;
			arg_166 = arg_165;
			arg_167 = arg_166;
			arg_168 = arg_167;
			arg_169 = arg_168;
			arg_170 = arg_169;
			arg_171 = arg_170;
			arg_172 = arg_171;
			arg_173 = arg_172;
			arg_174 = arg_173;
			arg_175 = arg_174;
			arg_176 = arg_175;
			arg_177 = arg_176;
			arg_178 = arg_177;
			arg_179 = arg_178;
			arg_180 = arg_179;
			arg_181 = arg_180;
			arg_182 = arg_181;
			arg_183 = arg_182;
			arg_184 = arg_183;
			arg_185 = arg_184;
			arg_186 = arg_185;
			arg_187 = arg_186;
			arg_188 = arg_187;
			arg_189 = arg_188;
			arg_190 = arg_189;
			arg_191 = arg_190;
			arg_192 = arg_191;
			arg_193 = arg_192;
			arg_194 = arg_193;
			arg_195 = arg_194;
			arg_196 = arg_195;
			arg_197 = arg_196;
			arg_198 = arg_197;
			arg_199 = arg_198;
			arg_200 = arg_199;
			arg_201 = arg_200;
			arg_202 = arg_201;
			arg_203 = arg_202;
			arg_204 = arg_203;
			arg_205 = arg_204;
			arg_206 = arg_205;
			arg_207 = arg_206;
			arg_208 = arg_207;
			arg_209 = arg_208;
			arg_210 = arg_209;
			arg_211 = arg_210;
			arg_212 = arg_211;
			arg_213 = arg_212;
			arg_214 = arg_213;
			arg_215 = arg_214;
			arg_216 = arg_215;
			arg_217 = arg_216;
			arg_218 = arg_217;
			arg_219 = arg_218;
			arg_220 = arg_219;
			arg_221 = arg_220;
			arg_222 = arg_221;
			arg_223 = arg_222;
			arg_224 = arg_223;
			arg_225 = arg_224;
			arg_226 = arg_225;
			arg_227 = arg_226;
			arg_228 = arg_227;
			arg_229 = arg_228;
			arg_230 = arg_229;
			arg_231 = arg_230;
			arg_232 = arg_231;
			arg_233 = arg_232;
			arg_234 = arg_233;
			arg_235 = arg_234;
			arg_236 = arg_235;
			arg_237 = arg_236;
			arg_238 = arg_237;
			arg_239 = arg_238;
			arg_240 = arg_239;
			arg_241 = arg_240;
			arg_242 = arg_241;
			arg_243 = arg_242;
			arg_244 = arg_243;
			arg_245 = arg_244;
			arg_246 = arg_245;
			arg_247 = arg_246;
			arg_248 = arg_247;
			arg_249 = arg_248;
			arg_250 = arg_249;
			arg_251 = arg_250;
			arg_252 = arg_251;
			arg_253 = arg_252;
			arg_254 = arg_253;
			arg_255 = arg_254;
			return arg_255;
		}
		#endregion

		#region Function call
		static void CallEmpty()
		{
			Empty();
		}

		static float ReturnValue()
		{
			return Ldc_R4();
		}
		#endregion

		#region Jump and comparisons
		static int Goto()
		{
			goto l2;
			l1:
			return 4;
			l2:
			goto l1;
			return 0;
		}

		static int Br()
		{
			goto l2;
			l1:
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			Empty();
			return 4;
			l2:
			goto l1;
			return 0;
		}

		static int Brfalse_S(bool b)
		{
			if (b == false)
			{
				return 1;
			}
			else
			{
				return 0;
			}
		}

		static int Brtrue_S(bool b)
		{
			if (b == true)
			{
				return 1;
			}
			else
			{
				return 0;
			}
		}

		static int Gt(int x)
		{
			if (x > 10)
			{
				return 5;
			}
			else
			{
				return 4;
			}
		}

		static int Lt(int x)
		{
			if (x < 10)
			{
				return 5;
			}
			else
			{
				return 4;
			}
		}

		static int Ge(int x)
		{
			if (x >= 10)
			{
				return 5;
			}
			else
			{
				return 4;
			}
		}

		static int Le(int x)
		{
			if (x <= 10)
			{
				return 5;
			}
			else
			{
				return 4;
			}
		}
		#endregion

		#region Maths
		static int Add(int a, int b)
		{
			return a + b;
		}

		static float AddFloat(float a, float b)
		{
			return a + b;
		}

		static int Sub(int a, int b)
		{
			return a - b;
		}

		static int Mul(int a, int b)
		{
			return a * b;
		}

		static int Div(int a, int b)
		{
			return a / b;
		}

		static uint Div_Un(uint a, uint b)
		{
			return a / b;
		}

		static bool IsEven(int a)
		{
			return a % 2 == 0;
		}

		static bool IsEven_Un(uint a)
		{
			return a % 2 == 0;
		}

		static int Negate(int x)
		{
			return -x;
		}

		static float NegateFloat(float f)
		{
			return -f;
		}
		#endregion

		#region Bitwise
		static int BitAnd(int a, int b)
		{
			return a & b;
		}

		static int BitOr(int a, int b)
		{
			return a | b;
		}

		static int BitXor(int a, int b)
		{
			return a ^ b;
		}

		static int BitNot(int a)
		{
			return ~a;
		}

		static int Shl(int a, int b)
		{
			return a << b;
		}

		static int Shr(int a, int b)
		{
			return a >> b;
		}
		#endregion

		#region Logic
		static bool LogicAnd(bool a, bool b)
		{
			return a && b;
		}

		static bool LogicOr(bool a, bool b)
		{
			return a || b;
		}

		static bool LogicNot(bool b)
		{
			return !b;
		}
		#endregion

		#region Objects
		static void CreateObject()
		{
			object o = new object();
		}
		#endregion
	}
}

