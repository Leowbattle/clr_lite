using System;
using System.Collections.Generic;
using System.Text;

namespace InterpreterTests
{
	class Tests
	{
		static void Empty() { }

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
	}
}
