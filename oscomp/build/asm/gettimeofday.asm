
/home/weihuan/Documents/testsuits-for-oskernel-preliminary/riscv-syscalls-testing/user/build/riscv64/gettimeofday:     file format elf64-littleriscv


Disassembly of section .text:

0000000000001000 <_start>:
.section .text.entry
.globl _start
_start:
    mv a0, sp
    1000:	850a                	mv	a0,sp
    tail __start_main
    1002:	a8e9                	j	10dc <__start_main>

0000000000001004 <test_gettimeofday>:
 * "start:[num], end:[num]"
 * "interval: [num]"	注：数字[num]的值应大于0
 * 测试失败时的输出：
 * "gettimeofday error."
 */
void test_gettimeofday() {
    1004:	7179                	addi	sp,sp,-48
	TEST_START(__func__);
    1006:	00001517          	auipc	a0,0x1
    100a:	f0250513          	addi	a0,a0,-254 # 1f08 <__clone+0x2c>
void test_gettimeofday() {
    100e:	f406                	sd	ra,40(sp)
    1010:	ec26                	sd	s1,24(sp)
    1012:	f022                	sd	s0,32(sp)
	TEST_START(__func__);
    1014:	33a000ef          	jal	ra,134e <puts>
    1018:	00001517          	auipc	a0,0x1
    101c:	fa050513          	addi	a0,a0,-96 # 1fb8 <__func__.0>
    1020:	32e000ef          	jal	ra,134e <puts>
    1024:	00001517          	auipc	a0,0x1
    1028:	efc50513          	addi	a0,a0,-260 # 1f20 <__clone+0x44>
    102c:	322000ef          	jal	ra,134e <puts>
	int test_ret1 = get_time();
    1030:	501000ef          	jal	ra,1d30 <get_time>
	volatile int i = 12500000;	// qemu时钟频率12500000
    1034:	00bec7b7          	lui	a5,0xbec
    1038:	c2078793          	addi	a5,a5,-992 # bebc20 <digits+0xbe9c50>
    103c:	c63e                	sw	a5,12(sp)
	while(i > 0) i--;
    103e:	47b2                	lw	a5,12(sp)
	int test_ret1 = get_time();
    1040:	0005049b          	sext.w	s1,a0
	while(i > 0) i--;
    1044:	2781                	sext.w	a5,a5
    1046:	00f05963          	blez	a5,1058 <test_gettimeofday+0x54>
    104a:	47b2                	lw	a5,12(sp)
    104c:	37fd                	addiw	a5,a5,-1
    104e:	c63e                	sw	a5,12(sp)
    1050:	47b2                	lw	a5,12(sp)
    1052:	2781                	sext.w	a5,a5
    1054:	fef04be3          	bgtz	a5,104a <test_gettimeofday+0x46>
	int test_ret2 = get_time();
    1058:	4d9000ef          	jal	ra,1d30 <get_time>
	if(test_ret1 > 0 && test_ret2 > 0){
    105c:	00905663          	blez	s1,1068 <test_gettimeofday+0x64>
	int test_ret2 = get_time();
    1060:	0005041b          	sext.w	s0,a0
	if(test_ret1 > 0 && test_ret2 > 0){
    1064:	02804d63          	bgtz	s0,109e <test_gettimeofday+0x9a>
		printf("gettimeofday success.\n");
		printf("start:%d, end:%d\n", test_ret1, test_ret2);
                printf("interval: %d\n", test_ret2 - test_ret1);
	}else{
		printf("gettimeofday error.\n");
    1068:	00001517          	auipc	a0,0x1
    106c:	f0850513          	addi	a0,a0,-248 # 1f70 <__clone+0x94>
    1070:	300000ef          	jal	ra,1370 <printf>
	}
	TEST_END(__func__);
    1074:	00001517          	auipc	a0,0x1
    1078:	f1450513          	addi	a0,a0,-236 # 1f88 <__clone+0xac>
    107c:	2d2000ef          	jal	ra,134e <puts>
    1080:	00001517          	auipc	a0,0x1
    1084:	f3850513          	addi	a0,a0,-200 # 1fb8 <__func__.0>
    1088:	2c6000ef          	jal	ra,134e <puts>
}
    108c:	7402                	ld	s0,32(sp)
    108e:	70a2                	ld	ra,40(sp)
    1090:	64e2                	ld	s1,24(sp)
	TEST_END(__func__);
    1092:	00001517          	auipc	a0,0x1
    1096:	e8e50513          	addi	a0,a0,-370 # 1f20 <__clone+0x44>
}
    109a:	6145                	addi	sp,sp,48
	TEST_END(__func__);
    109c:	ac4d                	j	134e <puts>
		printf("gettimeofday success.\n");
    109e:	00001517          	auipc	a0,0x1
    10a2:	e9250513          	addi	a0,a0,-366 # 1f30 <__clone+0x54>
    10a6:	2ca000ef          	jal	ra,1370 <printf>
		printf("start:%d, end:%d\n", test_ret1, test_ret2);
    10aa:	85a6                	mv	a1,s1
    10ac:	8622                	mv	a2,s0
    10ae:	00001517          	auipc	a0,0x1
    10b2:	e9a50513          	addi	a0,a0,-358 # 1f48 <__clone+0x6c>
    10b6:	2ba000ef          	jal	ra,1370 <printf>
                printf("interval: %d\n", test_ret2 - test_ret1);
    10ba:	409405bb          	subw	a1,s0,s1
    10be:	00001517          	auipc	a0,0x1
    10c2:	ea250513          	addi	a0,a0,-350 # 1f60 <__clone+0x84>
    10c6:	2aa000ef          	jal	ra,1370 <printf>
    10ca:	b76d                	j	1074 <test_gettimeofday+0x70>

00000000000010cc <main>:

int main(void) {
    10cc:	1141                	addi	sp,sp,-16
    10ce:	e406                	sd	ra,8(sp)
	test_gettimeofday();
    10d0:	f35ff0ef          	jal	ra,1004 <test_gettimeofday>
	return 0;
}
    10d4:	60a2                	ld	ra,8(sp)
    10d6:	4501                	li	a0,0
    10d8:	0141                	addi	sp,sp,16
    10da:	8082                	ret

00000000000010dc <__start_main>:
#include <unistd.h>

extern int main();

int __start_main(long *p)
{
    10dc:	85aa                	mv	a1,a0
	int argc = p[0];
	char **argv = (void *)(p+1);

	exit(main(argc, argv));
    10de:	4108                	lw	a0,0(a0)
{
    10e0:	1141                	addi	sp,sp,-16
	exit(main(argc, argv));
    10e2:	05a1                	addi	a1,a1,8
{
    10e4:	e406                	sd	ra,8(sp)
	exit(main(argc, argv));
    10e6:	fe7ff0ef          	jal	ra,10cc <main>
    10ea:	40b000ef          	jal	ra,1cf4 <exit>
	return 0;
}
    10ee:	60a2                	ld	ra,8(sp)
    10f0:	4501                	li	a0,0
    10f2:	0141                	addi	sp,sp,16
    10f4:	8082                	ret

00000000000010f6 <printint.constprop.0>:
    write(f, s, l);
}

static char digits[] = "0123456789abcdef";

static void printint(int xx, int base, int sign)
    10f6:	7179                	addi	sp,sp,-48
    10f8:	f406                	sd	ra,40(sp)
{
    char buf[16 + 1];
    int i;
    uint x;

    if (sign && (sign = xx < 0))
    10fa:	12054b63          	bltz	a0,1230 <printint.constprop.0+0x13a>

    buf[16] = 0;
    i = 15;
    do
    {
        buf[i--] = digits[x % base];
    10fe:	02b577bb          	remuw	a5,a0,a1
    1102:	00001617          	auipc	a2,0x1
    1106:	ece60613          	addi	a2,a2,-306 # 1fd0 <digits>
    buf[16] = 0;
    110a:	00010c23          	sb	zero,24(sp)
        buf[i--] = digits[x % base];
    110e:	0005871b          	sext.w	a4,a1
    1112:	1782                	slli	a5,a5,0x20
    1114:	9381                	srli	a5,a5,0x20
    1116:	97b2                	add	a5,a5,a2
    1118:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    111c:	02b5583b          	divuw	a6,a0,a1
        buf[i--] = digits[x % base];
    1120:	00f10ba3          	sb	a5,23(sp)
    } while ((x /= base) != 0);
    1124:	1cb56363          	bltu	a0,a1,12ea <printint.constprop.0+0x1f4>
        buf[i--] = digits[x % base];
    1128:	45b9                	li	a1,14
    112a:	02e877bb          	remuw	a5,a6,a4
    112e:	1782                	slli	a5,a5,0x20
    1130:	9381                	srli	a5,a5,0x20
    1132:	97b2                	add	a5,a5,a2
    1134:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    1138:	02e856bb          	divuw	a3,a6,a4
        buf[i--] = digits[x % base];
    113c:	00f10b23          	sb	a5,22(sp)
    } while ((x /= base) != 0);
    1140:	0ce86e63          	bltu	a6,a4,121c <printint.constprop.0+0x126>
        buf[i--] = digits[x % base];
    1144:	02e6f5bb          	remuw	a1,a3,a4
    } while ((x /= base) != 0);
    1148:	02e6d7bb          	divuw	a5,a3,a4
        buf[i--] = digits[x % base];
    114c:	1582                	slli	a1,a1,0x20
    114e:	9181                	srli	a1,a1,0x20
    1150:	95b2                	add	a1,a1,a2
    1152:	0005c583          	lbu	a1,0(a1)
    1156:	00b10aa3          	sb	a1,21(sp)
    } while ((x /= base) != 0);
    115a:	0007859b          	sext.w	a1,a5
    115e:	12e6ec63          	bltu	a3,a4,1296 <printint.constprop.0+0x1a0>
        buf[i--] = digits[x % base];
    1162:	02e7f6bb          	remuw	a3,a5,a4
    1166:	1682                	slli	a3,a3,0x20
    1168:	9281                	srli	a3,a3,0x20
    116a:	96b2                	add	a3,a3,a2
    116c:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    1170:	02e7d83b          	divuw	a6,a5,a4
        buf[i--] = digits[x % base];
    1174:	00d10a23          	sb	a3,20(sp)
    } while ((x /= base) != 0);
    1178:	12e5e863          	bltu	a1,a4,12a8 <printint.constprop.0+0x1b2>
        buf[i--] = digits[x % base];
    117c:	02e876bb          	remuw	a3,a6,a4
    1180:	1682                	slli	a3,a3,0x20
    1182:	9281                	srli	a3,a3,0x20
    1184:	96b2                	add	a3,a3,a2
    1186:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    118a:	02e855bb          	divuw	a1,a6,a4
        buf[i--] = digits[x % base];
    118e:	00d109a3          	sb	a3,19(sp)
    } while ((x /= base) != 0);
    1192:	12e86463          	bltu	a6,a4,12ba <printint.constprop.0+0x1c4>
        buf[i--] = digits[x % base];
    1196:	02e5f6bb          	remuw	a3,a1,a4
    119a:	1682                	slli	a3,a3,0x20
    119c:	9281                	srli	a3,a3,0x20
    119e:	96b2                	add	a3,a3,a2
    11a0:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11a4:	02e5d83b          	divuw	a6,a1,a4
        buf[i--] = digits[x % base];
    11a8:	00d10923          	sb	a3,18(sp)
    } while ((x /= base) != 0);
    11ac:	0ce5ec63          	bltu	a1,a4,1284 <printint.constprop.0+0x18e>
        buf[i--] = digits[x % base];
    11b0:	02e876bb          	remuw	a3,a6,a4
    11b4:	1682                	slli	a3,a3,0x20
    11b6:	9281                	srli	a3,a3,0x20
    11b8:	96b2                	add	a3,a3,a2
    11ba:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11be:	02e855bb          	divuw	a1,a6,a4
        buf[i--] = digits[x % base];
    11c2:	00d108a3          	sb	a3,17(sp)
    } while ((x /= base) != 0);
    11c6:	10e86963          	bltu	a6,a4,12d8 <printint.constprop.0+0x1e2>
        buf[i--] = digits[x % base];
    11ca:	02e5f6bb          	remuw	a3,a1,a4
    11ce:	1682                	slli	a3,a3,0x20
    11d0:	9281                	srli	a3,a3,0x20
    11d2:	96b2                	add	a3,a3,a2
    11d4:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11d8:	02e5d83b          	divuw	a6,a1,a4
        buf[i--] = digits[x % base];
    11dc:	00d10823          	sb	a3,16(sp)
    } while ((x /= base) != 0);
    11e0:	10e5e763          	bltu	a1,a4,12ee <printint.constprop.0+0x1f8>
        buf[i--] = digits[x % base];
    11e4:	02e876bb          	remuw	a3,a6,a4
    11e8:	1682                	slli	a3,a3,0x20
    11ea:	9281                	srli	a3,a3,0x20
    11ec:	96b2                	add	a3,a3,a2
    11ee:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11f2:	02e857bb          	divuw	a5,a6,a4
        buf[i--] = digits[x % base];
    11f6:	00d107a3          	sb	a3,15(sp)
    } while ((x /= base) != 0);
    11fa:	10e86363          	bltu	a6,a4,1300 <printint.constprop.0+0x20a>
        buf[i--] = digits[x % base];
    11fe:	1782                	slli	a5,a5,0x20
    1200:	9381                	srli	a5,a5,0x20
    1202:	97b2                	add	a5,a5,a2
    1204:	0007c783          	lbu	a5,0(a5)
    1208:	4599                	li	a1,6
    120a:	00f10723          	sb	a5,14(sp)

    if (sign)
    120e:	00055763          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    1212:	02d00793          	li	a5,45
    1216:	00f106a3          	sb	a5,13(sp)
        buf[i--] = digits[x % base];
    121a:	4595                	li	a1,5
    write(f, s, l);
    121c:	003c                	addi	a5,sp,8
    121e:	4641                	li	a2,16
    1220:	9e0d                	subw	a2,a2,a1
    1222:	4505                	li	a0,1
    1224:	95be                	add	a1,a1,a5
    1226:	27f000ef          	jal	ra,1ca4 <write>
    i++;
    if (i < 0)
        puts("printint error");
    out(stdout, buf + i, 16 - i);
}
    122a:	70a2                	ld	ra,40(sp)
    122c:	6145                	addi	sp,sp,48
    122e:	8082                	ret
        x = -xx;
    1230:	40a0083b          	negw	a6,a0
        buf[i--] = digits[x % base];
    1234:	02b877bb          	remuw	a5,a6,a1
    1238:	00001617          	auipc	a2,0x1
    123c:	d9860613          	addi	a2,a2,-616 # 1fd0 <digits>
    buf[16] = 0;
    1240:	00010c23          	sb	zero,24(sp)
        buf[i--] = digits[x % base];
    1244:	0005871b          	sext.w	a4,a1
    1248:	1782                	slli	a5,a5,0x20
    124a:	9381                	srli	a5,a5,0x20
    124c:	97b2                	add	a5,a5,a2
    124e:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    1252:	02b858bb          	divuw	a7,a6,a1
        buf[i--] = digits[x % base];
    1256:	00f10ba3          	sb	a5,23(sp)
    } while ((x /= base) != 0);
    125a:	06b86963          	bltu	a6,a1,12cc <printint.constprop.0+0x1d6>
        buf[i--] = digits[x % base];
    125e:	02e8f7bb          	remuw	a5,a7,a4
    1262:	1782                	slli	a5,a5,0x20
    1264:	9381                	srli	a5,a5,0x20
    1266:	97b2                	add	a5,a5,a2
    1268:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    126c:	02e8d6bb          	divuw	a3,a7,a4
        buf[i--] = digits[x % base];
    1270:	00f10b23          	sb	a5,22(sp)
    } while ((x /= base) != 0);
    1274:	ece8f8e3          	bgeu	a7,a4,1144 <printint.constprop.0+0x4e>
        buf[i--] = '-';
    1278:	02d00793          	li	a5,45
    127c:	00f10aa3          	sb	a5,21(sp)
        buf[i--] = digits[x % base];
    1280:	45b5                	li	a1,13
    1282:	bf69                	j	121c <printint.constprop.0+0x126>
    1284:	45a9                	li	a1,10
    if (sign)
    1286:	f8055be3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    128a:	02d00793          	li	a5,45
    128e:	00f108a3          	sb	a5,17(sp)
        buf[i--] = digits[x % base];
    1292:	45a5                	li	a1,9
    1294:	b761                	j	121c <printint.constprop.0+0x126>
    1296:	45b5                	li	a1,13
    if (sign)
    1298:	f80552e3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    129c:	02d00793          	li	a5,45
    12a0:	00f10a23          	sb	a5,20(sp)
        buf[i--] = digits[x % base];
    12a4:	45b1                	li	a1,12
    12a6:	bf9d                	j	121c <printint.constprop.0+0x126>
    12a8:	45b1                	li	a1,12
    if (sign)
    12aa:	f60559e3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    12ae:	02d00793          	li	a5,45
    12b2:	00f109a3          	sb	a5,19(sp)
        buf[i--] = digits[x % base];
    12b6:	45ad                	li	a1,11
    12b8:	b795                	j	121c <printint.constprop.0+0x126>
    12ba:	45ad                	li	a1,11
    if (sign)
    12bc:	f60550e3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    12c0:	02d00793          	li	a5,45
    12c4:	00f10923          	sb	a5,18(sp)
        buf[i--] = digits[x % base];
    12c8:	45a9                	li	a1,10
    12ca:	bf89                	j	121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    12cc:	02d00793          	li	a5,45
    12d0:	00f10b23          	sb	a5,22(sp)
        buf[i--] = digits[x % base];
    12d4:	45b9                	li	a1,14
    12d6:	b799                	j	121c <printint.constprop.0+0x126>
    12d8:	45a5                	li	a1,9
    if (sign)
    12da:	f40551e3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    12de:	02d00793          	li	a5,45
    12e2:	00f10823          	sb	a5,16(sp)
        buf[i--] = digits[x % base];
    12e6:	45a1                	li	a1,8
    12e8:	bf15                	j	121c <printint.constprop.0+0x126>
    i = 15;
    12ea:	45bd                	li	a1,15
    12ec:	bf05                	j	121c <printint.constprop.0+0x126>
        buf[i--] = digits[x % base];
    12ee:	45a1                	li	a1,8
    if (sign)
    12f0:	f20556e3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    12f4:	02d00793          	li	a5,45
    12f8:	00f107a3          	sb	a5,15(sp)
        buf[i--] = digits[x % base];
    12fc:	459d                	li	a1,7
    12fe:	bf39                	j	121c <printint.constprop.0+0x126>
    1300:	459d                	li	a1,7
    if (sign)
    1302:	f0055de3          	bgez	a0,121c <printint.constprop.0+0x126>
        buf[i--] = '-';
    1306:	02d00793          	li	a5,45
    130a:	00f10723          	sb	a5,14(sp)
        buf[i--] = digits[x % base];
    130e:	4599                	li	a1,6
    1310:	b731                	j	121c <printint.constprop.0+0x126>

0000000000001312 <getchar>:
{
    1312:	1101                	addi	sp,sp,-32
    read(stdin, &byte, 1);
    1314:	00f10593          	addi	a1,sp,15
    1318:	4605                	li	a2,1
    131a:	4501                	li	a0,0
{
    131c:	ec06                	sd	ra,24(sp)
    char byte = 0;
    131e:	000107a3          	sb	zero,15(sp)
    read(stdin, &byte, 1);
    1322:	179000ef          	jal	ra,1c9a <read>
}
    1326:	60e2                	ld	ra,24(sp)
    1328:	00f14503          	lbu	a0,15(sp)
    132c:	6105                	addi	sp,sp,32
    132e:	8082                	ret

0000000000001330 <putchar>:
{
    1330:	1101                	addi	sp,sp,-32
    1332:	87aa                	mv	a5,a0
    return write(stdout, &byte, 1);
    1334:	00f10593          	addi	a1,sp,15
    1338:	4605                	li	a2,1
    133a:	4505                	li	a0,1
{
    133c:	ec06                	sd	ra,24(sp)
    char byte = c;
    133e:	00f107a3          	sb	a5,15(sp)
    return write(stdout, &byte, 1);
    1342:	163000ef          	jal	ra,1ca4 <write>
}
    1346:	60e2                	ld	ra,24(sp)
    1348:	2501                	sext.w	a0,a0
    134a:	6105                	addi	sp,sp,32
    134c:	8082                	ret

000000000000134e <puts>:
{
    134e:	1141                	addi	sp,sp,-16
    1350:	e406                	sd	ra,8(sp)
    1352:	e022                	sd	s0,0(sp)
    1354:	842a                	mv	s0,a0
    r = -(write(stdout, s, strlen(s)) < 0);
    1356:	56a000ef          	jal	ra,18c0 <strlen>
    135a:	862a                	mv	a2,a0
    135c:	85a2                	mv	a1,s0
    135e:	4505                	li	a0,1
    1360:	145000ef          	jal	ra,1ca4 <write>
}
    1364:	60a2                	ld	ra,8(sp)
    1366:	6402                	ld	s0,0(sp)
    r = -(write(stdout, s, strlen(s)) < 0);
    1368:	957d                	srai	a0,a0,0x3f
    return r;
    136a:	2501                	sext.w	a0,a0
}
    136c:	0141                	addi	sp,sp,16
    136e:	8082                	ret

0000000000001370 <printf>:
    out(stdout, buf, i);
}

// Print to the console. only understands %d, %x, %p, %s.
void printf(const char *fmt, ...)
{
    1370:	7171                	addi	sp,sp,-176
    1372:	fc56                	sd	s5,56(sp)
    1374:	ed3e                	sd	a5,152(sp)
    buf[i++] = '0';
    1376:	7ae1                	lui	s5,0xffff8
    va_list ap;
    int cnt = 0, l = 0;
    char *a, *z, *s = (char *)fmt, str;
    int f = stdout;

    va_start(ap, fmt);
    1378:	18bc                	addi	a5,sp,120
{
    137a:	e8ca                	sd	s2,80(sp)
    137c:	e4ce                	sd	s3,72(sp)
    137e:	e0d2                	sd	s4,64(sp)
    1380:	f85a                	sd	s6,48(sp)
    1382:	f486                	sd	ra,104(sp)
    1384:	f0a2                	sd	s0,96(sp)
    1386:	eca6                	sd	s1,88(sp)
    1388:	fcae                	sd	a1,120(sp)
    138a:	e132                	sd	a2,128(sp)
    138c:	e536                	sd	a3,136(sp)
    138e:	e93a                	sd	a4,144(sp)
    1390:	f142                	sd	a6,160(sp)
    1392:	f546                	sd	a7,168(sp)
    va_start(ap, fmt);
    1394:	e03e                	sd	a5,0(sp)
    for (;;)
    {
        if (!*s)
            break;
        for (a = s; *s && *s != '%'; s++)
    1396:	02500913          	li	s2,37
        out(f, a, l);
        if (l)
            continue;
        if (s[1] == 0)
            break;
        switch (s[1])
    139a:	07300a13          	li	s4,115
        case 'p':
            printptr(va_arg(ap, uint64));
            break;
        case 's':
            if ((a = va_arg(ap, char *)) == 0)
                a = "(null)";
    139e:	00001b17          	auipc	s6,0x1
    13a2:	bfab0b13          	addi	s6,s6,-1030 # 1f98 <__clone+0xbc>
    buf[i++] = '0';
    13a6:	830aca93          	xori	s5,s5,-2000
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    13aa:	00001997          	auipc	s3,0x1
    13ae:	c2698993          	addi	s3,s3,-986 # 1fd0 <digits>
        if (!*s)
    13b2:	00054783          	lbu	a5,0(a0)
    13b6:	16078a63          	beqz	a5,152a <printf+0x1ba>
    13ba:	862a                	mv	a2,a0
        for (a = s; *s && *s != '%'; s++)
    13bc:	19278163          	beq	a5,s2,153e <printf+0x1ce>
    13c0:	00164783          	lbu	a5,1(a2)
    13c4:	0605                	addi	a2,a2,1
    13c6:	fbfd                	bnez	a5,13bc <printf+0x4c>
    13c8:	84b2                	mv	s1,a2
        l = z - a;
    13ca:	40a6043b          	subw	s0,a2,a0
    write(f, s, l);
    13ce:	85aa                	mv	a1,a0
    13d0:	8622                	mv	a2,s0
    13d2:	4505                	li	a0,1
    13d4:	0d1000ef          	jal	ra,1ca4 <write>
        if (l)
    13d8:	18041c63          	bnez	s0,1570 <printf+0x200>
        if (s[1] == 0)
    13dc:	0014c783          	lbu	a5,1(s1)
    13e0:	14078563          	beqz	a5,152a <printf+0x1ba>
        switch (s[1])
    13e4:	1d478063          	beq	a5,s4,15a4 <printf+0x234>
    13e8:	18fa6663          	bltu	s4,a5,1574 <printf+0x204>
    13ec:	06400713          	li	a4,100
    13f0:	1ae78063          	beq	a5,a4,1590 <printf+0x220>
    13f4:	07000713          	li	a4,112
    13f8:	1ce79963          	bne	a5,a4,15ca <printf+0x25a>
            printptr(va_arg(ap, uint64));
    13fc:	6702                	ld	a4,0(sp)
    buf[i++] = '0';
    13fe:	01511423          	sh	s5,8(sp)
    write(f, s, l);
    1402:	4649                	li	a2,18
            printptr(va_arg(ap, uint64));
    1404:	631c                	ld	a5,0(a4)
    1406:	0721                	addi	a4,a4,8
    1408:	e03a                	sd	a4,0(sp)
    for (j = 0; j < (sizeof(uint64) * 2); j++, x <<= 4)
    140a:	00479293          	slli	t0,a5,0x4
    140e:	00879f93          	slli	t6,a5,0x8
    1412:	00c79f13          	slli	t5,a5,0xc
    1416:	01079e93          	slli	t4,a5,0x10
    141a:	01479e13          	slli	t3,a5,0x14
    141e:	01879313          	slli	t1,a5,0x18
    1422:	01c79893          	slli	a7,a5,0x1c
    1426:	02479813          	slli	a6,a5,0x24
    142a:	02879513          	slli	a0,a5,0x28
    142e:	02c79593          	slli	a1,a5,0x2c
    1432:	03079693          	slli	a3,a5,0x30
    1436:	03479713          	slli	a4,a5,0x34
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    143a:	03c7d413          	srli	s0,a5,0x3c
    143e:	01c7d39b          	srliw	t2,a5,0x1c
    1442:	03c2d293          	srli	t0,t0,0x3c
    1446:	03cfdf93          	srli	t6,t6,0x3c
    144a:	03cf5f13          	srli	t5,t5,0x3c
    144e:	03cede93          	srli	t4,t4,0x3c
    1452:	03ce5e13          	srli	t3,t3,0x3c
    1456:	03c35313          	srli	t1,t1,0x3c
    145a:	03c8d893          	srli	a7,a7,0x3c
    145e:	03c85813          	srli	a6,a6,0x3c
    1462:	9171                	srli	a0,a0,0x3c
    1464:	91f1                	srli	a1,a1,0x3c
    1466:	92f1                	srli	a3,a3,0x3c
    1468:	9371                	srli	a4,a4,0x3c
    146a:	96ce                	add	a3,a3,s3
    146c:	974e                	add	a4,a4,s3
    146e:	944e                	add	s0,s0,s3
    1470:	92ce                	add	t0,t0,s3
    1472:	9fce                	add	t6,t6,s3
    1474:	9f4e                	add	t5,t5,s3
    1476:	9ece                	add	t4,t4,s3
    1478:	9e4e                	add	t3,t3,s3
    147a:	934e                	add	t1,t1,s3
    147c:	98ce                	add	a7,a7,s3
    147e:	93ce                	add	t2,t2,s3
    1480:	984e                	add	a6,a6,s3
    1482:	954e                	add	a0,a0,s3
    1484:	95ce                	add	a1,a1,s3
    1486:	0006c083          	lbu	ra,0(a3)
    148a:	0002c283          	lbu	t0,0(t0)
    148e:	00074683          	lbu	a3,0(a4)
    1492:	000fcf83          	lbu	t6,0(t6)
    1496:	000f4f03          	lbu	t5,0(t5)
    149a:	000ece83          	lbu	t4,0(t4)
    149e:	000e4e03          	lbu	t3,0(t3)
    14a2:	00034303          	lbu	t1,0(t1)
    14a6:	0008c883          	lbu	a7,0(a7)
    14aa:	0003c383          	lbu	t2,0(t2)
    14ae:	00084803          	lbu	a6,0(a6)
    14b2:	00054503          	lbu	a0,0(a0)
    14b6:	0005c583          	lbu	a1,0(a1)
    14ba:	00044403          	lbu	s0,0(s0)
    for (j = 0; j < (sizeof(uint64) * 2); j++, x <<= 4)
    14be:	03879713          	slli	a4,a5,0x38
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    14c2:	9371                	srli	a4,a4,0x3c
    14c4:	8bbd                	andi	a5,a5,15
    14c6:	974e                	add	a4,a4,s3
    14c8:	97ce                	add	a5,a5,s3
    14ca:	005105a3          	sb	t0,11(sp)
    14ce:	01f10623          	sb	t6,12(sp)
    14d2:	01e106a3          	sb	t5,13(sp)
    14d6:	01d10723          	sb	t4,14(sp)
    14da:	01c107a3          	sb	t3,15(sp)
    14de:	00610823          	sb	t1,16(sp)
    14e2:	011108a3          	sb	a7,17(sp)
    14e6:	00710923          	sb	t2,18(sp)
    14ea:	010109a3          	sb	a6,19(sp)
    14ee:	00a10a23          	sb	a0,20(sp)
    14f2:	00b10aa3          	sb	a1,21(sp)
    14f6:	00110b23          	sb	ra,22(sp)
    14fa:	00d10ba3          	sb	a3,23(sp)
    14fe:	00810523          	sb	s0,10(sp)
    1502:	00074703          	lbu	a4,0(a4)
    1506:	0007c783          	lbu	a5,0(a5)
    write(f, s, l);
    150a:	002c                	addi	a1,sp,8
    150c:	4505                	li	a0,1
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    150e:	00e10c23          	sb	a4,24(sp)
    1512:	00f10ca3          	sb	a5,25(sp)
    buf[i] = 0;
    1516:	00010d23          	sb	zero,26(sp)
    write(f, s, l);
    151a:	78a000ef          	jal	ra,1ca4 <write>
            // Print unknown % sequence to draw attention.
            putchar('%');
            putchar(s[1]);
            break;
        }
        s += 2;
    151e:	00248513          	addi	a0,s1,2
        if (!*s)
    1522:	00054783          	lbu	a5,0(a0)
    1526:	e8079ae3          	bnez	a5,13ba <printf+0x4a>
    }
    va_end(ap);
}
    152a:	70a6                	ld	ra,104(sp)
    152c:	7406                	ld	s0,96(sp)
    152e:	64e6                	ld	s1,88(sp)
    1530:	6946                	ld	s2,80(sp)
    1532:	69a6                	ld	s3,72(sp)
    1534:	6a06                	ld	s4,64(sp)
    1536:	7ae2                	ld	s5,56(sp)
    1538:	7b42                	ld	s6,48(sp)
    153a:	614d                	addi	sp,sp,176
    153c:	8082                	ret
        for (z = s; s[0] == '%' && s[1] == '%'; z++, s += 2)
    153e:	00064783          	lbu	a5,0(a2)
    1542:	84b2                	mv	s1,a2
    1544:	01278963          	beq	a5,s2,1556 <printf+0x1e6>
    1548:	b549                	j	13ca <printf+0x5a>
    154a:	0024c783          	lbu	a5,2(s1)
    154e:	0605                	addi	a2,a2,1
    1550:	0489                	addi	s1,s1,2
    1552:	e7279ce3          	bne	a5,s2,13ca <printf+0x5a>
    1556:	0014c783          	lbu	a5,1(s1)
    155a:	ff2788e3          	beq	a5,s2,154a <printf+0x1da>
        l = z - a;
    155e:	40a6043b          	subw	s0,a2,a0
    write(f, s, l);
    1562:	85aa                	mv	a1,a0
    1564:	8622                	mv	a2,s0
    1566:	4505                	li	a0,1
    1568:	73c000ef          	jal	ra,1ca4 <write>
        if (l)
    156c:	e60408e3          	beqz	s0,13dc <printf+0x6c>
    1570:	8526                	mv	a0,s1
    1572:	b581                	j	13b2 <printf+0x42>
        switch (s[1])
    1574:	07800713          	li	a4,120
    1578:	04e79963          	bne	a5,a4,15ca <printf+0x25a>
            printint(va_arg(ap, int), 16, 1);
    157c:	6782                	ld	a5,0(sp)
    157e:	45c1                	li	a1,16
    1580:	4388                	lw	a0,0(a5)
    1582:	07a1                	addi	a5,a5,8
    1584:	e03e                	sd	a5,0(sp)
    1586:	b71ff0ef          	jal	ra,10f6 <printint.constprop.0>
        s += 2;
    158a:	00248513          	addi	a0,s1,2
    158e:	bf51                	j	1522 <printf+0x1b2>
            printint(va_arg(ap, int), 10, 1);
    1590:	6782                	ld	a5,0(sp)
    1592:	45a9                	li	a1,10
    1594:	4388                	lw	a0,0(a5)
    1596:	07a1                	addi	a5,a5,8
    1598:	e03e                	sd	a5,0(sp)
    159a:	b5dff0ef          	jal	ra,10f6 <printint.constprop.0>
        s += 2;
    159e:	00248513          	addi	a0,s1,2
    15a2:	b741                	j	1522 <printf+0x1b2>
            if ((a = va_arg(ap, char *)) == 0)
    15a4:	6782                	ld	a5,0(sp)
    15a6:	6380                	ld	s0,0(a5)
    15a8:	07a1                	addi	a5,a5,8
    15aa:	e03e                	sd	a5,0(sp)
    15ac:	c031                	beqz	s0,15f0 <printf+0x280>
            l = strnlen(a, 200);
    15ae:	0c800593          	li	a1,200
    15b2:	8522                	mv	a0,s0
    15b4:	3f8000ef          	jal	ra,19ac <strnlen>
    write(f, s, l);
    15b8:	0005061b          	sext.w	a2,a0
    15bc:	85a2                	mv	a1,s0
    15be:	4505                	li	a0,1
    15c0:	6e4000ef          	jal	ra,1ca4 <write>
        s += 2;
    15c4:	00248513          	addi	a0,s1,2
    15c8:	bfa9                	j	1522 <printf+0x1b2>
    return write(stdout, &byte, 1);
    15ca:	4605                	li	a2,1
    15cc:	002c                	addi	a1,sp,8
    15ce:	4505                	li	a0,1
    char byte = c;
    15d0:	01210423          	sb	s2,8(sp)
    return write(stdout, &byte, 1);
    15d4:	6d0000ef          	jal	ra,1ca4 <write>
    char byte = c;
    15d8:	0014c783          	lbu	a5,1(s1)
    return write(stdout, &byte, 1);
    15dc:	4605                	li	a2,1
    15de:	002c                	addi	a1,sp,8
    15e0:	4505                	li	a0,1
    char byte = c;
    15e2:	00f10423          	sb	a5,8(sp)
    return write(stdout, &byte, 1);
    15e6:	6be000ef          	jal	ra,1ca4 <write>
        s += 2;
    15ea:	00248513          	addi	a0,s1,2
    15ee:	bf15                	j	1522 <printf+0x1b2>
                a = "(null)";
    15f0:	845a                	mv	s0,s6
    15f2:	bf75                	j	15ae <printf+0x23e>

00000000000015f4 <isspace>:
#define HIGHS (ONES * (UCHAR_MAX / 2 + 1))
#define HASZERO(x) (((x)-ONES) & ~(x)&HIGHS)

int isspace(int c)
{
    return c == ' ' || (unsigned)c - '\t' < 5;
    15f4:	02000793          	li	a5,32
    15f8:	00f50663          	beq	a0,a5,1604 <isspace+0x10>
    15fc:	355d                	addiw	a0,a0,-9
    15fe:	00553513          	sltiu	a0,a0,5
    1602:	8082                	ret
    1604:	4505                	li	a0,1
}
    1606:	8082                	ret

0000000000001608 <isdigit>:

int isdigit(int c)
{
    return (unsigned)c - '0' < 10;
    1608:	fd05051b          	addiw	a0,a0,-48
}
    160c:	00a53513          	sltiu	a0,a0,10
    1610:	8082                	ret

0000000000001612 <atoi>:
    return c == ' ' || (unsigned)c - '\t' < 5;
    1612:	02000613          	li	a2,32
    1616:	4591                	li	a1,4

int atoi(const char *s)
{
    int n = 0, neg = 0;
    while (isspace(*s))
    1618:	00054703          	lbu	a4,0(a0)
    return c == ' ' || (unsigned)c - '\t' < 5;
    161c:	ff77069b          	addiw	a3,a4,-9
    1620:	04c70d63          	beq	a4,a2,167a <atoi+0x68>
    1624:	0007079b          	sext.w	a5,a4
    1628:	04d5f963          	bgeu	a1,a3,167a <atoi+0x68>
        s++;
    switch (*s)
    162c:	02b00693          	li	a3,43
    1630:	04d70a63          	beq	a4,a3,1684 <atoi+0x72>
    1634:	02d00693          	li	a3,45
    1638:	06d70463          	beq	a4,a3,16a0 <atoi+0x8e>
        neg = 1;
    case '+':
        s++;
    }
    /* Compute n as a negative number to avoid overflow on INT_MIN */
    while (isdigit(*s))
    163c:	fd07859b          	addiw	a1,a5,-48
    1640:	4625                	li	a2,9
    1642:	873e                	mv	a4,a5
    1644:	86aa                	mv	a3,a0
    int n = 0, neg = 0;
    1646:	4e01                	li	t3,0
    while (isdigit(*s))
    1648:	04b66a63          	bltu	a2,a1,169c <atoi+0x8a>
    int n = 0, neg = 0;
    164c:	4501                	li	a0,0
    while (isdigit(*s))
    164e:	4825                	li	a6,9
    1650:	0016c603          	lbu	a2,1(a3)
        n = 10 * n - (*s++ - '0');
    1654:	0025179b          	slliw	a5,a0,0x2
    1658:	9d3d                	addw	a0,a0,a5
    165a:	fd07031b          	addiw	t1,a4,-48
    165e:	0015189b          	slliw	a7,a0,0x1
    while (isdigit(*s))
    1662:	fd06059b          	addiw	a1,a2,-48
        n = 10 * n - (*s++ - '0');
    1666:	0685                	addi	a3,a3,1
    1668:	4068853b          	subw	a0,a7,t1
    while (isdigit(*s))
    166c:	0006071b          	sext.w	a4,a2
    1670:	feb870e3          	bgeu	a6,a1,1650 <atoi+0x3e>
    return neg ? n : -n;
    1674:	000e0563          	beqz	t3,167e <atoi+0x6c>
}
    1678:	8082                	ret
        s++;
    167a:	0505                	addi	a0,a0,1
    167c:	bf71                	j	1618 <atoi+0x6>
    return neg ? n : -n;
    167e:	4113053b          	subw	a0,t1,a7
    1682:	8082                	ret
    while (isdigit(*s))
    1684:	00154783          	lbu	a5,1(a0)
    1688:	4625                	li	a2,9
        s++;
    168a:	00150693          	addi	a3,a0,1
    while (isdigit(*s))
    168e:	fd07859b          	addiw	a1,a5,-48
    1692:	0007871b          	sext.w	a4,a5
    int n = 0, neg = 0;
    1696:	4e01                	li	t3,0
    while (isdigit(*s))
    1698:	fab67ae3          	bgeu	a2,a1,164c <atoi+0x3a>
    169c:	4501                	li	a0,0
}
    169e:	8082                	ret
    while (isdigit(*s))
    16a0:	00154783          	lbu	a5,1(a0)
    16a4:	4625                	li	a2,9
        s++;
    16a6:	00150693          	addi	a3,a0,1
    while (isdigit(*s))
    16aa:	fd07859b          	addiw	a1,a5,-48
    16ae:	0007871b          	sext.w	a4,a5
    16b2:	feb665e3          	bltu	a2,a1,169c <atoi+0x8a>
        neg = 1;
    16b6:	4e05                	li	t3,1
    16b8:	bf51                	j	164c <atoi+0x3a>

00000000000016ba <memset>:

void *memset(void *dest, int c, size_t n)
{
    char *p = dest;
    for (int i = 0; i < n; ++i, *(p++) = c)
    16ba:	16060d63          	beqz	a2,1834 <memset+0x17a>
    16be:	40a007b3          	neg	a5,a0
    16c2:	8b9d                	andi	a5,a5,7
    16c4:	00778713          	addi	a4,a5,7
    16c8:	482d                	li	a6,11
    16ca:	0ff5f593          	zext.b	a1,a1
    16ce:	fff60693          	addi	a3,a2,-1
    16d2:	17076263          	bltu	a4,a6,1836 <memset+0x17c>
    16d6:	16e6ea63          	bltu	a3,a4,184a <memset+0x190>
    16da:	16078563          	beqz	a5,1844 <memset+0x18a>
    16de:	00b50023          	sb	a1,0(a0)
    16e2:	4705                	li	a4,1
    16e4:	00150e93          	addi	t4,a0,1
    16e8:	14e78c63          	beq	a5,a4,1840 <memset+0x186>
    16ec:	00b500a3          	sb	a1,1(a0)
    16f0:	4709                	li	a4,2
    16f2:	00250e93          	addi	t4,a0,2
    16f6:	14e78d63          	beq	a5,a4,1850 <memset+0x196>
    16fa:	00b50123          	sb	a1,2(a0)
    16fe:	470d                	li	a4,3
    1700:	00350e93          	addi	t4,a0,3
    1704:	12e78b63          	beq	a5,a4,183a <memset+0x180>
    1708:	00b501a3          	sb	a1,3(a0)
    170c:	4711                	li	a4,4
    170e:	00450e93          	addi	t4,a0,4
    1712:	14e78163          	beq	a5,a4,1854 <memset+0x19a>
    1716:	00b50223          	sb	a1,4(a0)
    171a:	4715                	li	a4,5
    171c:	00550e93          	addi	t4,a0,5
    1720:	12e78c63          	beq	a5,a4,1858 <memset+0x19e>
    1724:	00b502a3          	sb	a1,5(a0)
    1728:	471d                	li	a4,7
    172a:	00650e93          	addi	t4,a0,6
    172e:	12e79763          	bne	a5,a4,185c <memset+0x1a2>
    1732:	00750e93          	addi	t4,a0,7
    1736:	00b50323          	sb	a1,6(a0)
    173a:	4f1d                	li	t5,7
    173c:	00859713          	slli	a4,a1,0x8
    1740:	8f4d                	or	a4,a4,a1
    1742:	01059e13          	slli	t3,a1,0x10
    1746:	01c76e33          	or	t3,a4,t3
    174a:	01859313          	slli	t1,a1,0x18
    174e:	006e6333          	or	t1,t3,t1
    1752:	02059893          	slli	a7,a1,0x20
    1756:	011368b3          	or	a7,t1,a7
    175a:	02859813          	slli	a6,a1,0x28
    175e:	40f60333          	sub	t1,a2,a5
    1762:	0108e833          	or	a6,a7,a6
    1766:	03059693          	slli	a3,a1,0x30
    176a:	00d866b3          	or	a3,a6,a3
    176e:	03859713          	slli	a4,a1,0x38
    1772:	97aa                	add	a5,a5,a0
    1774:	ff837813          	andi	a6,t1,-8
    1778:	8f55                	or	a4,a4,a3
    177a:	00f806b3          	add	a3,a6,a5
    177e:	e398                	sd	a4,0(a5)
    1780:	07a1                	addi	a5,a5,8
    1782:	fed79ee3          	bne	a5,a3,177e <memset+0xc4>
    1786:	ff837693          	andi	a3,t1,-8
    178a:	00de87b3          	add	a5,t4,a3
    178e:	01e6873b          	addw	a4,a3,t5
    1792:	0ad30663          	beq	t1,a3,183e <memset+0x184>
    1796:	00b78023          	sb	a1,0(a5)
    179a:	0017069b          	addiw	a3,a4,1
    179e:	08c6fb63          	bgeu	a3,a2,1834 <memset+0x17a>
    17a2:	00b780a3          	sb	a1,1(a5)
    17a6:	0027069b          	addiw	a3,a4,2
    17aa:	08c6f563          	bgeu	a3,a2,1834 <memset+0x17a>
    17ae:	00b78123          	sb	a1,2(a5)
    17b2:	0037069b          	addiw	a3,a4,3
    17b6:	06c6ff63          	bgeu	a3,a2,1834 <memset+0x17a>
    17ba:	00b781a3          	sb	a1,3(a5)
    17be:	0047069b          	addiw	a3,a4,4
    17c2:	06c6f963          	bgeu	a3,a2,1834 <memset+0x17a>
    17c6:	00b78223          	sb	a1,4(a5)
    17ca:	0057069b          	addiw	a3,a4,5
    17ce:	06c6f363          	bgeu	a3,a2,1834 <memset+0x17a>
    17d2:	00b782a3          	sb	a1,5(a5)
    17d6:	0067069b          	addiw	a3,a4,6
    17da:	04c6fd63          	bgeu	a3,a2,1834 <memset+0x17a>
    17de:	00b78323          	sb	a1,6(a5)
    17e2:	0077069b          	addiw	a3,a4,7
    17e6:	04c6f763          	bgeu	a3,a2,1834 <memset+0x17a>
    17ea:	00b783a3          	sb	a1,7(a5)
    17ee:	0087069b          	addiw	a3,a4,8
    17f2:	04c6f163          	bgeu	a3,a2,1834 <memset+0x17a>
    17f6:	00b78423          	sb	a1,8(a5)
    17fa:	0097069b          	addiw	a3,a4,9
    17fe:	02c6fb63          	bgeu	a3,a2,1834 <memset+0x17a>
    1802:	00b784a3          	sb	a1,9(a5)
    1806:	00a7069b          	addiw	a3,a4,10
    180a:	02c6f563          	bgeu	a3,a2,1834 <memset+0x17a>
    180e:	00b78523          	sb	a1,10(a5)
    1812:	00b7069b          	addiw	a3,a4,11
    1816:	00c6ff63          	bgeu	a3,a2,1834 <memset+0x17a>
    181a:	00b785a3          	sb	a1,11(a5)
    181e:	00c7069b          	addiw	a3,a4,12
    1822:	00c6f963          	bgeu	a3,a2,1834 <memset+0x17a>
    1826:	00b78623          	sb	a1,12(a5)
    182a:	2735                	addiw	a4,a4,13
    182c:	00c77463          	bgeu	a4,a2,1834 <memset+0x17a>
    1830:	00b786a3          	sb	a1,13(a5)
        ;
    return dest;
}
    1834:	8082                	ret
    1836:	472d                	li	a4,11
    1838:	bd79                	j	16d6 <memset+0x1c>
    for (int i = 0; i < n; ++i, *(p++) = c)
    183a:	4f0d                	li	t5,3
    183c:	b701                	j	173c <memset+0x82>
    183e:	8082                	ret
    1840:	4f05                	li	t5,1
    1842:	bded                	j	173c <memset+0x82>
    1844:	8eaa                	mv	t4,a0
    1846:	4f01                	li	t5,0
    1848:	bdd5                	j	173c <memset+0x82>
    184a:	87aa                	mv	a5,a0
    184c:	4701                	li	a4,0
    184e:	b7a1                	j	1796 <memset+0xdc>
    1850:	4f09                	li	t5,2
    1852:	b5ed                	j	173c <memset+0x82>
    1854:	4f11                	li	t5,4
    1856:	b5dd                	j	173c <memset+0x82>
    1858:	4f15                	li	t5,5
    185a:	b5cd                	j	173c <memset+0x82>
    185c:	4f19                	li	t5,6
    185e:	bdf9                	j	173c <memset+0x82>

0000000000001860 <strcmp>:

int strcmp(const char *l, const char *r)
{
    for (; *l == *r && *l; l++, r++)
    1860:	00054783          	lbu	a5,0(a0)
    1864:	0005c703          	lbu	a4,0(a1)
    1868:	00e79863          	bne	a5,a4,1878 <strcmp+0x18>
    186c:	0505                	addi	a0,a0,1
    186e:	0585                	addi	a1,a1,1
    1870:	fbe5                	bnez	a5,1860 <strcmp>
    1872:	4501                	li	a0,0
        ;
    return *(unsigned char *)l - *(unsigned char *)r;
}
    1874:	9d19                	subw	a0,a0,a4
    1876:	8082                	ret
    return *(unsigned char *)l - *(unsigned char *)r;
    1878:	0007851b          	sext.w	a0,a5
    187c:	bfe5                	j	1874 <strcmp+0x14>

000000000000187e <strncmp>:

int strncmp(const char *_l, const char *_r, size_t n)
{
    const unsigned char *l = (void *)_l, *r = (void *)_r;
    if (!n--)
    187e:	ce05                	beqz	a2,18b6 <strncmp+0x38>
        return 0;
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    1880:	00054703          	lbu	a4,0(a0)
    1884:	0005c783          	lbu	a5,0(a1)
    1888:	cb0d                	beqz	a4,18ba <strncmp+0x3c>
    if (!n--)
    188a:	167d                	addi	a2,a2,-1
    188c:	00c506b3          	add	a3,a0,a2
    1890:	a819                	j	18a6 <strncmp+0x28>
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    1892:	00a68e63          	beq	a3,a0,18ae <strncmp+0x30>
    1896:	0505                	addi	a0,a0,1
    1898:	00e79b63          	bne	a5,a4,18ae <strncmp+0x30>
    189c:	00054703          	lbu	a4,0(a0)
        ;
    return *l - *r;
    18a0:	0005c783          	lbu	a5,0(a1)
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    18a4:	cb19                	beqz	a4,18ba <strncmp+0x3c>
    18a6:	0005c783          	lbu	a5,0(a1)
    18aa:	0585                	addi	a1,a1,1
    18ac:	f3fd                	bnez	a5,1892 <strncmp+0x14>
    return *l - *r;
    18ae:	0007051b          	sext.w	a0,a4
    18b2:	9d1d                	subw	a0,a0,a5
    18b4:	8082                	ret
        return 0;
    18b6:	4501                	li	a0,0
}
    18b8:	8082                	ret
    18ba:	4501                	li	a0,0
    return *l - *r;
    18bc:	9d1d                	subw	a0,a0,a5
    18be:	8082                	ret

00000000000018c0 <strlen>:
size_t strlen(const char *s)
{
    const char *a = s;
    typedef size_t __attribute__((__may_alias__)) word;
    const word *w;
    for (; (uintptr_t)s % SS; s++)
    18c0:	00757793          	andi	a5,a0,7
    18c4:	cf89                	beqz	a5,18de <strlen+0x1e>
    18c6:	87aa                	mv	a5,a0
    18c8:	a029                	j	18d2 <strlen+0x12>
    18ca:	0785                	addi	a5,a5,1
    18cc:	0077f713          	andi	a4,a5,7
    18d0:	cb01                	beqz	a4,18e0 <strlen+0x20>
        if (!*s)
    18d2:	0007c703          	lbu	a4,0(a5)
    18d6:	fb75                	bnez	a4,18ca <strlen+0xa>
    for (w = (const void *)s; !HASZERO(*w); w++)
        ;
    s = (const void *)w;
    for (; *s; s++)
        ;
    return s - a;
    18d8:	40a78533          	sub	a0,a5,a0
}
    18dc:	8082                	ret
    for (; (uintptr_t)s % SS; s++)
    18de:	87aa                	mv	a5,a0
    for (w = (const void *)s; !HASZERO(*w); w++)
    18e0:	6394                	ld	a3,0(a5)
    18e2:	00000597          	auipc	a1,0x0
    18e6:	6be5b583          	ld	a1,1726(a1) # 1fa0 <__clone+0xc4>
    18ea:	00000617          	auipc	a2,0x0
    18ee:	6be63603          	ld	a2,1726(a2) # 1fa8 <__clone+0xcc>
    18f2:	a019                	j	18f8 <strlen+0x38>
    18f4:	6794                	ld	a3,8(a5)
    18f6:	07a1                	addi	a5,a5,8
    18f8:	00b68733          	add	a4,a3,a1
    18fc:	fff6c693          	not	a3,a3
    1900:	8f75                	and	a4,a4,a3
    1902:	8f71                	and	a4,a4,a2
    1904:	db65                	beqz	a4,18f4 <strlen+0x34>
    for (; *s; s++)
    1906:	0007c703          	lbu	a4,0(a5)
    190a:	d779                	beqz	a4,18d8 <strlen+0x18>
    190c:	0017c703          	lbu	a4,1(a5)
    1910:	0785                	addi	a5,a5,1
    1912:	d379                	beqz	a4,18d8 <strlen+0x18>
    1914:	0017c703          	lbu	a4,1(a5)
    1918:	0785                	addi	a5,a5,1
    191a:	fb6d                	bnez	a4,190c <strlen+0x4c>
    191c:	bf75                	j	18d8 <strlen+0x18>

000000000000191e <memchr>:

void *memchr(const void *src, int c, size_t n)
{
    const unsigned char *s = src;
    c = (unsigned char)c;
    for (; ((uintptr_t)s & ALIGN) && n && *s != c; s++, n--)
    191e:	00757713          	andi	a4,a0,7
{
    1922:	87aa                	mv	a5,a0
    c = (unsigned char)c;
    1924:	0ff5f593          	zext.b	a1,a1
    for (; ((uintptr_t)s & ALIGN) && n && *s != c; s++, n--)
    1928:	cb19                	beqz	a4,193e <memchr+0x20>
    192a:	ce25                	beqz	a2,19a2 <memchr+0x84>
    192c:	0007c703          	lbu	a4,0(a5)
    1930:	04b70e63          	beq	a4,a1,198c <memchr+0x6e>
    1934:	0785                	addi	a5,a5,1
    1936:	0077f713          	andi	a4,a5,7
    193a:	167d                	addi	a2,a2,-1
    193c:	f77d                	bnez	a4,192a <memchr+0xc>
            ;
        s = (const void *)w;
    }
    for (; n && *s != c; s++, n--)
        ;
    return n ? (void *)s : 0;
    193e:	4501                	li	a0,0
    if (n && *s != c)
    1940:	c235                	beqz	a2,19a4 <memchr+0x86>
    1942:	0007c703          	lbu	a4,0(a5)
    1946:	04b70363          	beq	a4,a1,198c <memchr+0x6e>
        size_t k = ONES * c;
    194a:	00000517          	auipc	a0,0x0
    194e:	66653503          	ld	a0,1638(a0) # 1fb0 <__clone+0xd4>
        for (w = (const void *)s; n >= SS && !HASZERO(*w ^ k); w++, n -= SS)
    1952:	471d                	li	a4,7
        size_t k = ONES * c;
    1954:	02a58533          	mul	a0,a1,a0
        for (w = (const void *)s; n >= SS && !HASZERO(*w ^ k); w++, n -= SS)
    1958:	02c77a63          	bgeu	a4,a2,198c <memchr+0x6e>
    195c:	00000897          	auipc	a7,0x0
    1960:	6448b883          	ld	a7,1604(a7) # 1fa0 <__clone+0xc4>
    1964:	00000817          	auipc	a6,0x0
    1968:	64483803          	ld	a6,1604(a6) # 1fa8 <__clone+0xcc>
    196c:	431d                	li	t1,7
    196e:	a029                	j	1978 <memchr+0x5a>
    1970:	1661                	addi	a2,a2,-8
    1972:	07a1                	addi	a5,a5,8
    1974:	02c37963          	bgeu	t1,a2,19a6 <memchr+0x88>
    1978:	6398                	ld	a4,0(a5)
    197a:	8f29                	xor	a4,a4,a0
    197c:	011706b3          	add	a3,a4,a7
    1980:	fff74713          	not	a4,a4
    1984:	8f75                	and	a4,a4,a3
    1986:	01077733          	and	a4,a4,a6
    198a:	d37d                	beqz	a4,1970 <memchr+0x52>
    198c:	853e                	mv	a0,a5
    198e:	97b2                	add	a5,a5,a2
    1990:	a021                	j	1998 <memchr+0x7a>
    for (; n && *s != c; s++, n--)
    1992:	0505                	addi	a0,a0,1
    1994:	00f50763          	beq	a0,a5,19a2 <memchr+0x84>
    1998:	00054703          	lbu	a4,0(a0)
    199c:	feb71be3          	bne	a4,a1,1992 <memchr+0x74>
    19a0:	8082                	ret
    return n ? (void *)s : 0;
    19a2:	4501                	li	a0,0
}
    19a4:	8082                	ret
    return n ? (void *)s : 0;
    19a6:	4501                	li	a0,0
    for (; n && *s != c; s++, n--)
    19a8:	f275                	bnez	a2,198c <memchr+0x6e>
}
    19aa:	8082                	ret

00000000000019ac <strnlen>:

size_t strnlen(const char *s, size_t n)
{
    19ac:	1101                	addi	sp,sp,-32
    19ae:	e822                	sd	s0,16(sp)
    const char *p = memchr(s, 0, n);
    19b0:	862e                	mv	a2,a1
{
    19b2:	842e                	mv	s0,a1
    const char *p = memchr(s, 0, n);
    19b4:	4581                	li	a1,0
{
    19b6:	e426                	sd	s1,8(sp)
    19b8:	ec06                	sd	ra,24(sp)
    19ba:	84aa                	mv	s1,a0
    const char *p = memchr(s, 0, n);
    19bc:	f63ff0ef          	jal	ra,191e <memchr>
    return p ? p - s : n;
    19c0:	c519                	beqz	a0,19ce <strnlen+0x22>
}
    19c2:	60e2                	ld	ra,24(sp)
    19c4:	6442                	ld	s0,16(sp)
    return p ? p - s : n;
    19c6:	8d05                	sub	a0,a0,s1
}
    19c8:	64a2                	ld	s1,8(sp)
    19ca:	6105                	addi	sp,sp,32
    19cc:	8082                	ret
    19ce:	60e2                	ld	ra,24(sp)
    return p ? p - s : n;
    19d0:	8522                	mv	a0,s0
}
    19d2:	6442                	ld	s0,16(sp)
    19d4:	64a2                	ld	s1,8(sp)
    19d6:	6105                	addi	sp,sp,32
    19d8:	8082                	ret

00000000000019da <strcpy>:
char *strcpy(char *restrict d, const char *s)
{
    typedef size_t __attribute__((__may_alias__)) word;
    word *wd;
    const word *ws;
    if ((uintptr_t)s % SS == (uintptr_t)d % SS)
    19da:	00b547b3          	xor	a5,a0,a1
    19de:	8b9d                	andi	a5,a5,7
    19e0:	eb95                	bnez	a5,1a14 <strcpy+0x3a>
    {
        for (; (uintptr_t)s % SS; s++, d++)
    19e2:	0075f793          	andi	a5,a1,7
    19e6:	e7b1                	bnez	a5,1a32 <strcpy+0x58>
            if (!(*d = *s))
                return d;
        wd = (void *)d;
        ws = (const void *)s;
        for (; !HASZERO(*ws); *wd++ = *ws++)
    19e8:	6198                	ld	a4,0(a1)
    19ea:	00000617          	auipc	a2,0x0
    19ee:	5b663603          	ld	a2,1462(a2) # 1fa0 <__clone+0xc4>
    19f2:	00000817          	auipc	a6,0x0
    19f6:	5b683803          	ld	a6,1462(a6) # 1fa8 <__clone+0xcc>
    19fa:	a029                	j	1a04 <strcpy+0x2a>
    19fc:	e118                	sd	a4,0(a0)
    19fe:	6598                	ld	a4,8(a1)
    1a00:	05a1                	addi	a1,a1,8
    1a02:	0521                	addi	a0,a0,8
    1a04:	00c707b3          	add	a5,a4,a2
    1a08:	fff74693          	not	a3,a4
    1a0c:	8ff5                	and	a5,a5,a3
    1a0e:	0107f7b3          	and	a5,a5,a6
    1a12:	d7ed                	beqz	a5,19fc <strcpy+0x22>
            ;
        d = (void *)wd;
        s = (const void *)ws;
    }
    for (; (*d = *s); s++, d++)
    1a14:	0005c783          	lbu	a5,0(a1)
    1a18:	00f50023          	sb	a5,0(a0)
    1a1c:	c785                	beqz	a5,1a44 <strcpy+0x6a>
    1a1e:	0015c783          	lbu	a5,1(a1)
    1a22:	0505                	addi	a0,a0,1
    1a24:	0585                	addi	a1,a1,1
    1a26:	00f50023          	sb	a5,0(a0)
    1a2a:	fbf5                	bnez	a5,1a1e <strcpy+0x44>
        ;
    return d;
}
    1a2c:	8082                	ret
        for (; (uintptr_t)s % SS; s++, d++)
    1a2e:	0505                	addi	a0,a0,1
    1a30:	df45                	beqz	a4,19e8 <strcpy+0xe>
            if (!(*d = *s))
    1a32:	0005c783          	lbu	a5,0(a1)
        for (; (uintptr_t)s % SS; s++, d++)
    1a36:	0585                	addi	a1,a1,1
    1a38:	0075f713          	andi	a4,a1,7
            if (!(*d = *s))
    1a3c:	00f50023          	sb	a5,0(a0)
    1a40:	f7fd                	bnez	a5,1a2e <strcpy+0x54>
}
    1a42:	8082                	ret
    1a44:	8082                	ret

0000000000001a46 <strncpy>:
char *strncpy(char *restrict d, const char *s, size_t n)
{
    typedef size_t __attribute__((__may_alias__)) word;
    word *wd;
    const word *ws;
    if (((uintptr_t)s & ALIGN) == ((uintptr_t)d & ALIGN))
    1a46:	00b547b3          	xor	a5,a0,a1
    1a4a:	8b9d                	andi	a5,a5,7
    1a4c:	1a079863          	bnez	a5,1bfc <strncpy+0x1b6>
    {
        for (; ((uintptr_t)s & ALIGN) && n && (*d = *s); n--, s++, d++)
    1a50:	0075f793          	andi	a5,a1,7
    1a54:	16078463          	beqz	a5,1bbc <strncpy+0x176>
    1a58:	ea01                	bnez	a2,1a68 <strncpy+0x22>
    1a5a:	a421                	j	1c62 <strncpy+0x21c>
    1a5c:	167d                	addi	a2,a2,-1
    1a5e:	0505                	addi	a0,a0,1
    1a60:	14070e63          	beqz	a4,1bbc <strncpy+0x176>
    1a64:	1a060863          	beqz	a2,1c14 <strncpy+0x1ce>
    1a68:	0005c783          	lbu	a5,0(a1)
    1a6c:	0585                	addi	a1,a1,1
    1a6e:	0075f713          	andi	a4,a1,7
    1a72:	00f50023          	sb	a5,0(a0)
    1a76:	f3fd                	bnez	a5,1a5c <strncpy+0x16>
    1a78:	4805                	li	a6,1
    1a7a:	1a061863          	bnez	a2,1c2a <strncpy+0x1e4>
    1a7e:	40a007b3          	neg	a5,a0
    1a82:	8b9d                	andi	a5,a5,7
    1a84:	4681                	li	a3,0
    1a86:	18061a63          	bnez	a2,1c1a <strncpy+0x1d4>
    1a8a:	00778713          	addi	a4,a5,7
    1a8e:	45ad                	li	a1,11
    1a90:	18b76363          	bltu	a4,a1,1c16 <strncpy+0x1d0>
    1a94:	1ae6eb63          	bltu	a3,a4,1c4a <strncpy+0x204>
    1a98:	1a078363          	beqz	a5,1c3e <strncpy+0x1f8>
    for (int i = 0; i < n; ++i, *(p++) = c)
    1a9c:	00050023          	sb	zero,0(a0)
    1aa0:	4685                	li	a3,1
    1aa2:	00150713          	addi	a4,a0,1
    1aa6:	18d78f63          	beq	a5,a3,1c44 <strncpy+0x1fe>
    1aaa:	000500a3          	sb	zero,1(a0)
    1aae:	4689                	li	a3,2
    1ab0:	00250713          	addi	a4,a0,2
    1ab4:	18d78e63          	beq	a5,a3,1c50 <strncpy+0x20a>
    1ab8:	00050123          	sb	zero,2(a0)
    1abc:	468d                	li	a3,3
    1abe:	00350713          	addi	a4,a0,3
    1ac2:	16d78c63          	beq	a5,a3,1c3a <strncpy+0x1f4>
    1ac6:	000501a3          	sb	zero,3(a0)
    1aca:	4691                	li	a3,4
    1acc:	00450713          	addi	a4,a0,4
    1ad0:	18d78263          	beq	a5,a3,1c54 <strncpy+0x20e>
    1ad4:	00050223          	sb	zero,4(a0)
    1ad8:	4695                	li	a3,5
    1ada:	00550713          	addi	a4,a0,5
    1ade:	16d78d63          	beq	a5,a3,1c58 <strncpy+0x212>
    1ae2:	000502a3          	sb	zero,5(a0)
    1ae6:	469d                	li	a3,7
    1ae8:	00650713          	addi	a4,a0,6
    1aec:	16d79863          	bne	a5,a3,1c5c <strncpy+0x216>
    1af0:	00750713          	addi	a4,a0,7
    1af4:	00050323          	sb	zero,6(a0)
    1af8:	40f80833          	sub	a6,a6,a5
    1afc:	ff887593          	andi	a1,a6,-8
    1b00:	97aa                	add	a5,a5,a0
    1b02:	95be                	add	a1,a1,a5
    1b04:	0007b023          	sd	zero,0(a5)
    1b08:	07a1                	addi	a5,a5,8
    1b0a:	feb79de3          	bne	a5,a1,1b04 <strncpy+0xbe>
    1b0e:	ff887593          	andi	a1,a6,-8
    1b12:	9ead                	addw	a3,a3,a1
    1b14:	00b707b3          	add	a5,a4,a1
    1b18:	12b80863          	beq	a6,a1,1c48 <strncpy+0x202>
    1b1c:	00078023          	sb	zero,0(a5)
    1b20:	0016871b          	addiw	a4,a3,1
    1b24:	0ec77863          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b28:	000780a3          	sb	zero,1(a5)
    1b2c:	0026871b          	addiw	a4,a3,2
    1b30:	0ec77263          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b34:	00078123          	sb	zero,2(a5)
    1b38:	0036871b          	addiw	a4,a3,3
    1b3c:	0cc77c63          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b40:	000781a3          	sb	zero,3(a5)
    1b44:	0046871b          	addiw	a4,a3,4
    1b48:	0cc77663          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b4c:	00078223          	sb	zero,4(a5)
    1b50:	0056871b          	addiw	a4,a3,5
    1b54:	0cc77063          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b58:	000782a3          	sb	zero,5(a5)
    1b5c:	0066871b          	addiw	a4,a3,6
    1b60:	0ac77a63          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b64:	00078323          	sb	zero,6(a5)
    1b68:	0076871b          	addiw	a4,a3,7
    1b6c:	0ac77463          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b70:	000783a3          	sb	zero,7(a5)
    1b74:	0086871b          	addiw	a4,a3,8
    1b78:	08c77e63          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b7c:	00078423          	sb	zero,8(a5)
    1b80:	0096871b          	addiw	a4,a3,9
    1b84:	08c77863          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b88:	000784a3          	sb	zero,9(a5)
    1b8c:	00a6871b          	addiw	a4,a3,10
    1b90:	08c77263          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1b94:	00078523          	sb	zero,10(a5)
    1b98:	00b6871b          	addiw	a4,a3,11
    1b9c:	06c77c63          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1ba0:	000785a3          	sb	zero,11(a5)
    1ba4:	00c6871b          	addiw	a4,a3,12
    1ba8:	06c77663          	bgeu	a4,a2,1c14 <strncpy+0x1ce>
    1bac:	00078623          	sb	zero,12(a5)
    1bb0:	26b5                	addiw	a3,a3,13
    1bb2:	06c6f163          	bgeu	a3,a2,1c14 <strncpy+0x1ce>
    1bb6:	000786a3          	sb	zero,13(a5)
    1bba:	8082                	ret
            ;
        if (!n || !*s)
    1bbc:	c645                	beqz	a2,1c64 <strncpy+0x21e>
    1bbe:	0005c783          	lbu	a5,0(a1)
    1bc2:	ea078be3          	beqz	a5,1a78 <strncpy+0x32>
            goto tail;
        wd = (void *)d;
        ws = (const void *)s;
        for (; n >= sizeof(size_t) && !HASZERO(*ws); n -= sizeof(size_t), ws++, wd++)
    1bc6:	479d                	li	a5,7
    1bc8:	02c7ff63          	bgeu	a5,a2,1c06 <strncpy+0x1c0>
    1bcc:	00000897          	auipc	a7,0x0
    1bd0:	3d48b883          	ld	a7,980(a7) # 1fa0 <__clone+0xc4>
    1bd4:	00000817          	auipc	a6,0x0
    1bd8:	3d483803          	ld	a6,980(a6) # 1fa8 <__clone+0xcc>
    1bdc:	431d                	li	t1,7
    1bde:	6198                	ld	a4,0(a1)
    1be0:	011707b3          	add	a5,a4,a7
    1be4:	fff74693          	not	a3,a4
    1be8:	8ff5                	and	a5,a5,a3
    1bea:	0107f7b3          	and	a5,a5,a6
    1bee:	ef81                	bnez	a5,1c06 <strncpy+0x1c0>
            *wd = *ws;
    1bf0:	e118                	sd	a4,0(a0)
        for (; n >= sizeof(size_t) && !HASZERO(*ws); n -= sizeof(size_t), ws++, wd++)
    1bf2:	1661                	addi	a2,a2,-8
    1bf4:	05a1                	addi	a1,a1,8
    1bf6:	0521                	addi	a0,a0,8
    1bf8:	fec363e3          	bltu	t1,a2,1bde <strncpy+0x198>
        d = (void *)wd;
        s = (const void *)ws;
    }
    for (; n && (*d = *s); n--, s++, d++)
    1bfc:	e609                	bnez	a2,1c06 <strncpy+0x1c0>
    1bfe:	a08d                	j	1c60 <strncpy+0x21a>
    1c00:	167d                	addi	a2,a2,-1
    1c02:	0505                	addi	a0,a0,1
    1c04:	ca01                	beqz	a2,1c14 <strncpy+0x1ce>
    1c06:	0005c783          	lbu	a5,0(a1)
    1c0a:	0585                	addi	a1,a1,1
    1c0c:	00f50023          	sb	a5,0(a0)
    1c10:	fbe5                	bnez	a5,1c00 <strncpy+0x1ba>
        ;
tail:
    1c12:	b59d                	j	1a78 <strncpy+0x32>
    memset(d, 0, n);
    return d;
}
    1c14:	8082                	ret
    1c16:	472d                	li	a4,11
    1c18:	bdb5                	j	1a94 <strncpy+0x4e>
    1c1a:	00778713          	addi	a4,a5,7
    1c1e:	45ad                	li	a1,11
    1c20:	fff60693          	addi	a3,a2,-1
    1c24:	e6b778e3          	bgeu	a4,a1,1a94 <strncpy+0x4e>
    1c28:	b7fd                	j	1c16 <strncpy+0x1d0>
    1c2a:	40a007b3          	neg	a5,a0
    1c2e:	8832                	mv	a6,a2
    1c30:	8b9d                	andi	a5,a5,7
    1c32:	4681                	li	a3,0
    1c34:	e4060be3          	beqz	a2,1a8a <strncpy+0x44>
    1c38:	b7cd                	j	1c1a <strncpy+0x1d4>
    for (int i = 0; i < n; ++i, *(p++) = c)
    1c3a:	468d                	li	a3,3
    1c3c:	bd75                	j	1af8 <strncpy+0xb2>
    1c3e:	872a                	mv	a4,a0
    1c40:	4681                	li	a3,0
    1c42:	bd5d                	j	1af8 <strncpy+0xb2>
    1c44:	4685                	li	a3,1
    1c46:	bd4d                	j	1af8 <strncpy+0xb2>
    1c48:	8082                	ret
    1c4a:	87aa                	mv	a5,a0
    1c4c:	4681                	li	a3,0
    1c4e:	b5f9                	j	1b1c <strncpy+0xd6>
    1c50:	4689                	li	a3,2
    1c52:	b55d                	j	1af8 <strncpy+0xb2>
    1c54:	4691                	li	a3,4
    1c56:	b54d                	j	1af8 <strncpy+0xb2>
    1c58:	4695                	li	a3,5
    1c5a:	bd79                	j	1af8 <strncpy+0xb2>
    1c5c:	4699                	li	a3,6
    1c5e:	bd69                	j	1af8 <strncpy+0xb2>
    1c60:	8082                	ret
    1c62:	8082                	ret
    1c64:	8082                	ret

0000000000001c66 <open>:
#include <unistd.h>

#include "syscall.h"

int open(const char *path, int flags)
{
    1c66:	87aa                	mv	a5,a0
    1c68:	862e                	mv	a2,a1
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
    register long a7 __asm__("a7") = n;
    1c6a:	03800893          	li	a7,56
    register long a0 __asm__("a0") = a;
    1c6e:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1c72:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    1c74:	4689                	li	a3,2
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1c76:	00000073          	ecall
    return syscall(SYS_openat, AT_FDCWD, path, flags, O_RDWR);
}
    1c7a:	2501                	sext.w	a0,a0
    1c7c:	8082                	ret

0000000000001c7e <openat>:
    register long a7 __asm__("a7") = n;
    1c7e:	03800893          	li	a7,56
    register long a3 __asm__("a3") = d;
    1c82:	18000693          	li	a3,384
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1c86:	00000073          	ecall

int openat(int dirfd,const char *path, int flags)
{
    return syscall(SYS_openat, dirfd, path, flags, 0600);
}
    1c8a:	2501                	sext.w	a0,a0
    1c8c:	8082                	ret

0000000000001c8e <close>:
    register long a7 __asm__("a7") = n;
    1c8e:	03900893          	li	a7,57
    __asm_syscall("r"(a7), "0"(a0))
    1c92:	00000073          	ecall

int close(int fd)
{
    return syscall(SYS_close, fd);
}
    1c96:	2501                	sext.w	a0,a0
    1c98:	8082                	ret

0000000000001c9a <read>:
    register long a7 __asm__("a7") = n;
    1c9a:	03f00893          	li	a7,63
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1c9e:	00000073          	ecall

ssize_t read(int fd, void *buf, size_t len)
{
    return syscall(SYS_read, fd, buf, len);
}
    1ca2:	8082                	ret

0000000000001ca4 <write>:
    register long a7 __asm__("a7") = n;
    1ca4:	04000893          	li	a7,64
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1ca8:	00000073          	ecall

ssize_t write(int fd, const void *buf, size_t len)
{
    return syscall(SYS_write, fd, buf, len);
}
    1cac:	8082                	ret

0000000000001cae <getpid>:
    register long a7 __asm__("a7") = n;
    1cae:	0ac00893          	li	a7,172
    __asm_syscall("r"(a7))
    1cb2:	00000073          	ecall

pid_t getpid(void)
{
    return syscall(SYS_getpid);
}
    1cb6:	2501                	sext.w	a0,a0
    1cb8:	8082                	ret

0000000000001cba <getppid>:
    register long a7 __asm__("a7") = n;
    1cba:	0ad00893          	li	a7,173
    __asm_syscall("r"(a7))
    1cbe:	00000073          	ecall

pid_t getppid(void)
{
    return syscall(SYS_getppid);
}
    1cc2:	2501                	sext.w	a0,a0
    1cc4:	8082                	ret

0000000000001cc6 <sched_yield>:
    register long a7 __asm__("a7") = n;
    1cc6:	07c00893          	li	a7,124
    __asm_syscall("r"(a7))
    1cca:	00000073          	ecall

int sched_yield(void)
{
    return syscall(SYS_sched_yield);
}
    1cce:	2501                	sext.w	a0,a0
    1cd0:	8082                	ret

0000000000001cd2 <fork>:
    register long a7 __asm__("a7") = n;
    1cd2:	0dc00893          	li	a7,220
    register long a0 __asm__("a0") = a;
    1cd6:	4545                	li	a0,17
    register long a1 __asm__("a1") = b;
    1cd8:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1cda:	00000073          	ecall

pid_t fork(void)
{
    return syscall(SYS_clone, SIGCHLD, 0);
}
    1cde:	2501                	sext.w	a0,a0
    1ce0:	8082                	ret

0000000000001ce2 <clone>:

pid_t clone(int (*fn)(void *arg), void *arg, void *stack, size_t stack_size, unsigned long flags)
{
    1ce2:	85b2                	mv	a1,a2
    1ce4:	863a                	mv	a2,a4
    if (stack)
    1ce6:	c191                	beqz	a1,1cea <clone+0x8>
	stack += stack_size;
    1ce8:	95b6                	add	a1,a1,a3

    return __clone(fn, stack, flags, NULL, NULL, NULL);
    1cea:	4781                	li	a5,0
    1cec:	4701                	li	a4,0
    1cee:	4681                	li	a3,0
    1cf0:	2601                	sext.w	a2,a2
    1cf2:	a2ed                	j	1edc <__clone>

0000000000001cf4 <exit>:
    register long a7 __asm__("a7") = n;
    1cf4:	05d00893          	li	a7,93
    __asm_syscall("r"(a7), "0"(a0))
    1cf8:	00000073          	ecall
    //return syscall(SYS_clone, fn, stack, flags, NULL, NULL, NULL);
}
void exit(int code)
{
    syscall(SYS_exit, code);
}
    1cfc:	8082                	ret

0000000000001cfe <waitpid>:
    register long a7 __asm__("a7") = n;
    1cfe:	10400893          	li	a7,260
    register long a3 __asm__("a3") = d;
    1d02:	4681                	li	a3,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1d04:	00000073          	ecall

int waitpid(int pid, int *code, int options)
{
    return syscall(SYS_wait4, pid, code, options, 0);
}
    1d08:	2501                	sext.w	a0,a0
    1d0a:	8082                	ret

0000000000001d0c <exec>:
    register long a7 __asm__("a7") = n;
    1d0c:	0dd00893          	li	a7,221
    __asm_syscall("r"(a7), "0"(a0))
    1d10:	00000073          	ecall

int exec(char *name)
{
    return syscall(SYS_execve, name);
}
    1d14:	2501                	sext.w	a0,a0
    1d16:	8082                	ret

0000000000001d18 <execve>:
    register long a7 __asm__("a7") = n;
    1d18:	0dd00893          	li	a7,221
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1d1c:	00000073          	ecall

int execve(const char *name, char *const argv[], char *const argp[])
{
    return syscall(SYS_execve, name, argv, argp);
}
    1d20:	2501                	sext.w	a0,a0
    1d22:	8082                	ret

0000000000001d24 <times>:
    register long a7 __asm__("a7") = n;
    1d24:	09900893          	li	a7,153
    __asm_syscall("r"(a7), "0"(a0))
    1d28:	00000073          	ecall

int times(void *mytimes)
{
	return syscall(SYS_times, mytimes);
}
    1d2c:	2501                	sext.w	a0,a0
    1d2e:	8082                	ret

0000000000001d30 <get_time>:

int64 get_time()
{
    1d30:	1141                	addi	sp,sp,-16
    register long a7 __asm__("a7") = n;
    1d32:	0a900893          	li	a7,169
    register long a0 __asm__("a0") = a;
    1d36:	850a                	mv	a0,sp
    register long a1 __asm__("a1") = b;
    1d38:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1d3a:	00000073          	ecall
    TimeVal time;
    int err = sys_get_time(&time, 0);
    if (err == 0)
    1d3e:	2501                	sext.w	a0,a0
    1d40:	ed09                	bnez	a0,1d5a <get_time+0x2a>
    {
        return ((time.sec & 0xffff) * 1000 + time.usec / 1000);
    1d42:	67a2                	ld	a5,8(sp)
    1d44:	3e800713          	li	a4,1000
    1d48:	00015503          	lhu	a0,0(sp)
    1d4c:	02e7d7b3          	divu	a5,a5,a4
    1d50:	02e50533          	mul	a0,a0,a4
    1d54:	953e                	add	a0,a0,a5
    }
    else
    {
        return -1;
    }
}
    1d56:	0141                	addi	sp,sp,16
    1d58:	8082                	ret
        return -1;
    1d5a:	557d                	li	a0,-1
    1d5c:	bfed                	j	1d56 <get_time+0x26>

0000000000001d5e <sys_get_time>:
    register long a7 __asm__("a7") = n;
    1d5e:	0a900893          	li	a7,169
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1d62:	00000073          	ecall

int sys_get_time(TimeVal *ts, int tz)
{
    return syscall(SYS_gettimeofday, ts, tz);
}
    1d66:	2501                	sext.w	a0,a0
    1d68:	8082                	ret

0000000000001d6a <time>:
    register long a7 __asm__("a7") = n;
    1d6a:	42600893          	li	a7,1062
    __asm_syscall("r"(a7), "0"(a0))
    1d6e:	00000073          	ecall

int time(unsigned long *tloc)
{
    return syscall(SYS_time, tloc);
}
    1d72:	2501                	sext.w	a0,a0
    1d74:	8082                	ret

0000000000001d76 <sleep>:

int sleep(unsigned long long time)
{
    1d76:	1141                	addi	sp,sp,-16
    TimeVal tv = {.sec = time, .usec = 0};
    1d78:	e02a                	sd	a0,0(sp)
    register long a0 __asm__("a0") = a;
    1d7a:	850a                	mv	a0,sp
    1d7c:	e402                	sd	zero,8(sp)
    register long a7 __asm__("a7") = n;
    1d7e:	06500893          	li	a7,101
    register long a1 __asm__("a1") = b;
    1d82:	85aa                	mv	a1,a0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1d84:	00000073          	ecall
    if (syscall(SYS_nanosleep, &tv, &tv)) return tv.sec;
    1d88:	e501                	bnez	a0,1d90 <sleep+0x1a>
    return 0;
    1d8a:	4501                	li	a0,0
}
    1d8c:	0141                	addi	sp,sp,16
    1d8e:	8082                	ret
    if (syscall(SYS_nanosleep, &tv, &tv)) return tv.sec;
    1d90:	4502                	lw	a0,0(sp)
}
    1d92:	0141                	addi	sp,sp,16
    1d94:	8082                	ret

0000000000001d96 <set_priority>:
    register long a7 __asm__("a7") = n;
    1d96:	08c00893          	li	a7,140
    __asm_syscall("r"(a7), "0"(a0))
    1d9a:	00000073          	ecall

int set_priority(int prio)
{
    return syscall(SYS_setpriority, prio);
}
    1d9e:	2501                	sext.w	a0,a0
    1da0:	8082                	ret

0000000000001da2 <mmap>:
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
    register long a7 __asm__("a7") = n;
    1da2:	0de00893          	li	a7,222
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    register long a4 __asm__("a4") = e;
    register long a5 __asm__("a5") = f;
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5))
    1da6:	00000073          	ecall

void *mmap(void *start, size_t len, int prot, int flags, int fd, off_t off)
{
    return syscall(SYS_mmap, start, len, prot, flags, fd, off);
}
    1daa:	8082                	ret

0000000000001dac <munmap>:
    register long a7 __asm__("a7") = n;
    1dac:	0d700893          	li	a7,215
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1db0:	00000073          	ecall

int munmap(void *start, size_t len)
{
    return syscall(SYS_munmap, start, len);
}
    1db4:	2501                	sext.w	a0,a0
    1db6:	8082                	ret

0000000000001db8 <wait>:

int wait(int *code)
{
    1db8:	85aa                	mv	a1,a0
    register long a7 __asm__("a7") = n;
    1dba:	10400893          	li	a7,260
    register long a0 __asm__("a0") = a;
    1dbe:	557d                	li	a0,-1
    register long a2 __asm__("a2") = c;
    1dc0:	4601                	li	a2,0
    register long a3 __asm__("a3") = d;
    1dc2:	4681                	li	a3,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1dc4:	00000073          	ecall
    return waitpid((int)-1, code, 0);
}
    1dc8:	2501                	sext.w	a0,a0
    1dca:	8082                	ret

0000000000001dcc <spawn>:
    register long a7 __asm__("a7") = n;
    1dcc:	19000893          	li	a7,400
    __asm_syscall("r"(a7), "0"(a0))
    1dd0:	00000073          	ecall

int spawn(char *file)
{
    return syscall(SYS_spawn, file);
}
    1dd4:	2501                	sext.w	a0,a0
    1dd6:	8082                	ret

0000000000001dd8 <mailread>:
    register long a7 __asm__("a7") = n;
    1dd8:	19100893          	li	a7,401
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1ddc:	00000073          	ecall

int mailread(void *buf, int len)
{
    return syscall(SYS_mailread, buf, len);
}
    1de0:	2501                	sext.w	a0,a0
    1de2:	8082                	ret

0000000000001de4 <mailwrite>:
    register long a7 __asm__("a7") = n;
    1de4:	19200893          	li	a7,402
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1de8:	00000073          	ecall

int mailwrite(int pid, void *buf, int len)
{
    return syscall(SYS_mailwrite, pid, buf, len);
}
    1dec:	2501                	sext.w	a0,a0
    1dee:	8082                	ret

0000000000001df0 <fstat>:
    register long a7 __asm__("a7") = n;
    1df0:	05000893          	li	a7,80
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1df4:	00000073          	ecall

int fstat(int fd, struct kstat *st)
{
    return syscall(SYS_fstat, fd, st);
}
    1df8:	2501                	sext.w	a0,a0
    1dfa:	8082                	ret

0000000000001dfc <sys_linkat>:
    register long a4 __asm__("a4") = e;
    1dfc:	1702                	slli	a4,a4,0x20
    register long a7 __asm__("a7") = n;
    1dfe:	02500893          	li	a7,37
    register long a4 __asm__("a4") = e;
    1e02:	9301                	srli	a4,a4,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1e04:	00000073          	ecall

int sys_linkat(int olddirfd, char *oldpath, int newdirfd, char *newpath, unsigned int flags)
{
    return syscall(SYS_linkat, olddirfd, oldpath, newdirfd, newpath, flags);
}
    1e08:	2501                	sext.w	a0,a0
    1e0a:	8082                	ret

0000000000001e0c <sys_unlinkat>:
    register long a2 __asm__("a2") = c;
    1e0c:	1602                	slli	a2,a2,0x20
    register long a7 __asm__("a7") = n;
    1e0e:	02300893          	li	a7,35
    register long a2 __asm__("a2") = c;
    1e12:	9201                	srli	a2,a2,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e14:	00000073          	ecall

int sys_unlinkat(int dirfd, char *path, unsigned int flags)
{
    return syscall(SYS_unlinkat, dirfd, path, flags);
}
    1e18:	2501                	sext.w	a0,a0
    1e1a:	8082                	ret

0000000000001e1c <link>:

int link(char *old_path, char *new_path)
{
    1e1c:	87aa                	mv	a5,a0
    1e1e:	86ae                	mv	a3,a1
    register long a7 __asm__("a7") = n;
    1e20:	02500893          	li	a7,37
    register long a0 __asm__("a0") = a;
    1e24:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1e28:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    1e2a:	f9c00613          	li	a2,-100
    register long a4 __asm__("a4") = e;
    1e2e:	4701                	li	a4,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1e30:	00000073          	ecall
    return sys_linkat(AT_FDCWD, old_path, AT_FDCWD, new_path, 0);
}
    1e34:	2501                	sext.w	a0,a0
    1e36:	8082                	ret

0000000000001e38 <unlink>:

int unlink(char *path)
{
    1e38:	85aa                	mv	a1,a0
    register long a7 __asm__("a7") = n;
    1e3a:	02300893          	li	a7,35
    register long a0 __asm__("a0") = a;
    1e3e:	f9c00513          	li	a0,-100
    register long a2 __asm__("a2") = c;
    1e42:	4601                	li	a2,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e44:	00000073          	ecall
    return sys_unlinkat(AT_FDCWD, path, 0);
}
    1e48:	2501                	sext.w	a0,a0
    1e4a:	8082                	ret

0000000000001e4c <uname>:
    register long a7 __asm__("a7") = n;
    1e4c:	0a000893          	li	a7,160
    __asm_syscall("r"(a7), "0"(a0))
    1e50:	00000073          	ecall

int uname(void *buf)
{
    return syscall(SYS_uname, buf);
}
    1e54:	2501                	sext.w	a0,a0
    1e56:	8082                	ret

0000000000001e58 <brk>:
    register long a7 __asm__("a7") = n;
    1e58:	0d600893          	li	a7,214
    __asm_syscall("r"(a7), "0"(a0))
    1e5c:	00000073          	ecall

int brk(void *addr)
{
    return syscall(SYS_brk, addr);
}
    1e60:	2501                	sext.w	a0,a0
    1e62:	8082                	ret

0000000000001e64 <getcwd>:
    register long a7 __asm__("a7") = n;
    1e64:	48c5                	li	a7,17
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e66:	00000073          	ecall

char *getcwd(char *buf, size_t size){
    return syscall(SYS_getcwd, buf, size);
}
    1e6a:	8082                	ret

0000000000001e6c <chdir>:
    register long a7 __asm__("a7") = n;
    1e6c:	03100893          	li	a7,49
    __asm_syscall("r"(a7), "0"(a0))
    1e70:	00000073          	ecall

int chdir(const char *path){
    return syscall(SYS_chdir, path);
}
    1e74:	2501                	sext.w	a0,a0
    1e76:	8082                	ret

0000000000001e78 <mkdir>:

int mkdir(const char *path, mode_t mode){
    1e78:	862e                	mv	a2,a1
    1e7a:	87aa                	mv	a5,a0
    register long a2 __asm__("a2") = c;
    1e7c:	1602                	slli	a2,a2,0x20
    register long a7 __asm__("a7") = n;
    1e7e:	02200893          	li	a7,34
    register long a0 __asm__("a0") = a;
    1e82:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1e86:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    1e88:	9201                	srli	a2,a2,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e8a:	00000073          	ecall
    return syscall(SYS_mkdirat, AT_FDCWD, path, mode);
}
    1e8e:	2501                	sext.w	a0,a0
    1e90:	8082                	ret

0000000000001e92 <getdents>:
    register long a7 __asm__("a7") = n;
    1e92:	03d00893          	li	a7,61
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e96:	00000073          	ecall

int getdents(int fd, struct linux_dirent64 *dirp64, unsigned long len){
    //return syscall(SYS_getdents64, fd, dirp64, len);
    return syscall(SYS_getdents64, fd, dirp64, len);
}
    1e9a:	2501                	sext.w	a0,a0
    1e9c:	8082                	ret

0000000000001e9e <pipe>:
    register long a7 __asm__("a7") = n;
    1e9e:	03b00893          	li	a7,59
    register long a1 __asm__("a1") = b;
    1ea2:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1ea4:	00000073          	ecall

int pipe(int fd[2]){
    return syscall(SYS_pipe2, fd, 0);
}
    1ea8:	2501                	sext.w	a0,a0
    1eaa:	8082                	ret

0000000000001eac <dup>:
    register long a7 __asm__("a7") = n;
    1eac:	48dd                	li	a7,23
    __asm_syscall("r"(a7), "0"(a0))
    1eae:	00000073          	ecall

int dup(int fd){
    return syscall(SYS_dup, fd);
}
    1eb2:	2501                	sext.w	a0,a0
    1eb4:	8082                	ret

0000000000001eb6 <dup2>:
    register long a7 __asm__("a7") = n;
    1eb6:	48e1                	li	a7,24
    register long a2 __asm__("a2") = c;
    1eb8:	4601                	li	a2,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1eba:	00000073          	ecall

int dup2(int old, int new){
    return syscall(SYS_dup3, old, new, 0);
}
    1ebe:	2501                	sext.w	a0,a0
    1ec0:	8082                	ret

0000000000001ec2 <mount>:
    register long a7 __asm__("a7") = n;
    1ec2:	02800893          	li	a7,40
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1ec6:	00000073          	ecall

int mount(const char *special, const char *dir, const char *fstype, unsigned long flags, const void *data)
{
        return syscall(SYS_mount, special, dir, fstype, flags, data);
}
    1eca:	2501                	sext.w	a0,a0
    1ecc:	8082                	ret

0000000000001ece <umount>:
    register long a7 __asm__("a7") = n;
    1ece:	02700893          	li	a7,39
    register long a1 __asm__("a1") = b;
    1ed2:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1ed4:	00000073          	ecall

int umount(const char *special)
{
        return syscall(SYS_umount2, special, 0);
}
    1ed8:	2501                	sext.w	a0,a0
    1eda:	8082                	ret

0000000000001edc <__clone>:

.global __clone
.type  __clone, %function
__clone:
	# Save func and arg to stack
	addi a1, a1, -16
    1edc:	15c1                	addi	a1,a1,-16
	sd a0, 0(a1)
    1ede:	e188                	sd	a0,0(a1)
	sd a3, 8(a1)
    1ee0:	e594                	sd	a3,8(a1)

	# Call SYS_clone
	mv a0, a2
    1ee2:	8532                	mv	a0,a2
	mv a2, a4
    1ee4:	863a                	mv	a2,a4
	mv a3, a5
    1ee6:	86be                	mv	a3,a5
	mv a4, a6
    1ee8:	8742                	mv	a4,a6
	li a7, 220 # SYS_clone
    1eea:	0dc00893          	li	a7,220
	ecall
    1eee:	00000073          	ecall

	beqz a0, 1f
    1ef2:	c111                	beqz	a0,1ef6 <__clone+0x1a>
	# Parent
	ret
    1ef4:	8082                	ret

	# Child
1:      ld a1, 0(sp)
    1ef6:	6582                	ld	a1,0(sp)
	ld a0, 8(sp)
    1ef8:	6522                	ld	a0,8(sp)
	jalr a1
    1efa:	9582                	jalr	a1

	# Exit
	li a7, 93 # SYS_exit
    1efc:	05d00893          	li	a7,93
	ecall
    1f00:	00000073          	ecall
