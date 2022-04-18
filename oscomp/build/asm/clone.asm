
/home/weihuan/Documents/testsuits-for-oskernel-preliminary/riscv-syscalls-testing/user/build/riscv64/clone:     file format elf64-littleriscv


Disassembly of section .text:

0000000000001000 <_start>:
.section .text.entry
.globl _start
_start:
    mv a0, sp
    1000:	850a                	mv	a0,sp
    tail __start_main
    1002:	a0f5                	j	10ee <__start_main>

0000000000001004 <child_func>:
#include "unistd.h"

size_t stack[1024] = {0};
static int child_pid;

static int child_func(void){
    1004:	1141                	addi	sp,sp,-16
    printf("  Child says successfully!\n");
    1006:	00001517          	auipc	a0,0x1
    100a:	f2250513          	addi	a0,a0,-222 # 1f28 <__clone+0x28>
static int child_func(void){
    100e:	e406                	sd	ra,8(sp)
    printf("  Child says successfully!\n");
    1010:	372000ef          	jal	ra,1382 <printf>
    return 0;
}
    1014:	60a2                	ld	ra,8(sp)
    1016:	4501                	li	a0,0
    1018:	0141                	addi	sp,sp,16
    101a:	8082                	ret

000000000000101c <test_clone>:

void test_clone(void){
    101c:	1101                	addi	sp,sp,-32
    TEST_START(__func__);
    101e:	00001517          	auipc	a0,0x1
    1022:	f2a50513          	addi	a0,a0,-214 # 1f48 <__clone+0x48>
void test_clone(void){
    1026:	ec06                	sd	ra,24(sp)
    1028:	e822                	sd	s0,16(sp)
    TEST_START(__func__);
    102a:	336000ef          	jal	ra,1360 <puts>
    102e:	00003517          	auipc	a0,0x3
    1032:	fda50513          	addi	a0,a0,-38 # 4008 <__func__.0>
    1036:	32a000ef          	jal	ra,1360 <puts>
    103a:	00001517          	auipc	a0,0x1
    103e:	f2650513          	addi	a0,a0,-218 # 1f60 <__clone+0x60>
    1042:	31e000ef          	jal	ra,1360 <puts>
    int wstatus;
    child_pid = clone(child_func, NULL, stack, 1024, SIGCHLD);
    1046:	4745                	li	a4,17
    1048:	40000693          	li	a3,1024
    104c:	00001617          	auipc	a2,0x1
    1050:	fb460613          	addi	a2,a2,-76 # 2000 <stack>
    1054:	4581                	li	a1,0
    1056:	00000517          	auipc	a0,0x0
    105a:	fae50513          	addi	a0,a0,-82 # 1004 <child_func>
    105e:	4a9000ef          	jal	ra,1d06 <clone>
    1062:	00003417          	auipc	s0,0x3
    1066:	f9e40413          	addi	s0,s0,-98 # 4000 <child_pid>
    106a:	c008                	sw	a0,0(s0)
    assert(child_pid != -1);
    106c:	57fd                	li	a5,-1
    106e:	04f50863          	beq	a0,a5,10be <test_clone+0xa2>
    if (child_pid == 0){
    1072:	e90d                	bnez	a0,10a4 <test_clone+0x88>
	exit(0);
    1074:	4a5000ef          	jal	ra,1d18 <exit>
	    printf("clone process successfully.\npid:%d\n", child_pid);
	else
	    printf("clone process error.\n");
    }

    TEST_END(__func__);
    1078:	00001517          	auipc	a0,0x1
    107c:	f5850513          	addi	a0,a0,-168 # 1fd0 <__clone+0xd0>
    1080:	2e0000ef          	jal	ra,1360 <puts>
    1084:	00003517          	auipc	a0,0x3
    1088:	f8450513          	addi	a0,a0,-124 # 4008 <__func__.0>
    108c:	2d4000ef          	jal	ra,1360 <puts>
    1090:	00001517          	auipc	a0,0x1
    1094:	ed050513          	addi	a0,a0,-304 # 1f60 <__clone+0x60>
    1098:	2c8000ef          	jal	ra,1360 <puts>
}
    109c:	60e2                	ld	ra,24(sp)
    109e:	6442                	ld	s0,16(sp)
    10a0:	6105                	addi	sp,sp,32
    10a2:	8082                	ret
	if(wait(&wstatus) == child_pid)
    10a4:	0068                	addi	a0,sp,12
    10a6:	537000ef          	jal	ra,1ddc <wait>
    10aa:	401c                	lw	a5,0(s0)
    10ac:	02f50163          	beq	a0,a5,10ce <test_clone+0xb2>
	    printf("clone process error.\n");
    10b0:	00001517          	auipc	a0,0x1
    10b4:	f0850513          	addi	a0,a0,-248 # 1fb8 <__clone+0xb8>
    10b8:	2ca000ef          	jal	ra,1382 <printf>
    10bc:	bf75                	j	1078 <test_clone+0x5c>
    assert(child_pid != -1);
    10be:	00001517          	auipc	a0,0x1
    10c2:	eb250513          	addi	a0,a0,-334 # 1f70 <__clone+0x70>
    10c6:	540000ef          	jal	ra,1606 <panic>
    if (child_pid == 0){
    10ca:	4008                	lw	a0,0(s0)
    10cc:	b75d                	j	1072 <test_clone+0x56>
	    printf("clone process successfully.\npid:%d\n", child_pid);
    10ce:	85aa                	mv	a1,a0
    10d0:	00001517          	auipc	a0,0x1
    10d4:	ec050513          	addi	a0,a0,-320 # 1f90 <__clone+0x90>
    10d8:	2aa000ef          	jal	ra,1382 <printf>
    10dc:	bf71                	j	1078 <test_clone+0x5c>

00000000000010de <main>:

int main(void){
    10de:	1141                	addi	sp,sp,-16
    10e0:	e406                	sd	ra,8(sp)
    test_clone();
    10e2:	f3bff0ef          	jal	ra,101c <test_clone>
    return 0;
}
    10e6:	60a2                	ld	ra,8(sp)
    10e8:	4501                	li	a0,0
    10ea:	0141                	addi	sp,sp,16
    10ec:	8082                	ret

00000000000010ee <__start_main>:
#include <unistd.h>

extern int main();

int __start_main(long *p)
{
    10ee:	85aa                	mv	a1,a0
	int argc = p[0];
	char **argv = (void *)(p+1);

	exit(main(argc, argv));
    10f0:	4108                	lw	a0,0(a0)
{
    10f2:	1141                	addi	sp,sp,-16
	exit(main(argc, argv));
    10f4:	05a1                	addi	a1,a1,8
{
    10f6:	e406                	sd	ra,8(sp)
	exit(main(argc, argv));
    10f8:	fe7ff0ef          	jal	ra,10de <main>
    10fc:	41d000ef          	jal	ra,1d18 <exit>
	return 0;
}
    1100:	60a2                	ld	ra,8(sp)
    1102:	4501                	li	a0,0
    1104:	0141                	addi	sp,sp,16
    1106:	8082                	ret

0000000000001108 <printint.constprop.0>:
    write(f, s, l);
}

static char digits[] = "0123456789abcdef";

static void printint(int xx, int base, int sign)
    1108:	7179                	addi	sp,sp,-48
    110a:	f406                	sd	ra,40(sp)
{
    char buf[16 + 1];
    int i;
    uint x;

    if (sign && (sign = xx < 0))
    110c:	12054b63          	bltz	a0,1242 <printint.constprop.0+0x13a>

    buf[16] = 0;
    i = 15;
    do
    {
        buf[i--] = digits[x % base];
    1110:	02b577bb          	remuw	a5,a0,a1
    1114:	00003617          	auipc	a2,0x3
    1118:	f0460613          	addi	a2,a2,-252 # 4018 <digits>
    buf[16] = 0;
    111c:	00010c23          	sb	zero,24(sp)
        buf[i--] = digits[x % base];
    1120:	0005871b          	sext.w	a4,a1
    1124:	1782                	slli	a5,a5,0x20
    1126:	9381                	srli	a5,a5,0x20
    1128:	97b2                	add	a5,a5,a2
    112a:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    112e:	02b5583b          	divuw	a6,a0,a1
        buf[i--] = digits[x % base];
    1132:	00f10ba3          	sb	a5,23(sp)
    } while ((x /= base) != 0);
    1136:	1cb56363          	bltu	a0,a1,12fc <printint.constprop.0+0x1f4>
        buf[i--] = digits[x % base];
    113a:	45b9                	li	a1,14
    113c:	02e877bb          	remuw	a5,a6,a4
    1140:	1782                	slli	a5,a5,0x20
    1142:	9381                	srli	a5,a5,0x20
    1144:	97b2                	add	a5,a5,a2
    1146:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    114a:	02e856bb          	divuw	a3,a6,a4
        buf[i--] = digits[x % base];
    114e:	00f10b23          	sb	a5,22(sp)
    } while ((x /= base) != 0);
    1152:	0ce86e63          	bltu	a6,a4,122e <printint.constprop.0+0x126>
        buf[i--] = digits[x % base];
    1156:	02e6f5bb          	remuw	a1,a3,a4
    } while ((x /= base) != 0);
    115a:	02e6d7bb          	divuw	a5,a3,a4
        buf[i--] = digits[x % base];
    115e:	1582                	slli	a1,a1,0x20
    1160:	9181                	srli	a1,a1,0x20
    1162:	95b2                	add	a1,a1,a2
    1164:	0005c583          	lbu	a1,0(a1)
    1168:	00b10aa3          	sb	a1,21(sp)
    } while ((x /= base) != 0);
    116c:	0007859b          	sext.w	a1,a5
    1170:	12e6ec63          	bltu	a3,a4,12a8 <printint.constprop.0+0x1a0>
        buf[i--] = digits[x % base];
    1174:	02e7f6bb          	remuw	a3,a5,a4
    1178:	1682                	slli	a3,a3,0x20
    117a:	9281                	srli	a3,a3,0x20
    117c:	96b2                	add	a3,a3,a2
    117e:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    1182:	02e7d83b          	divuw	a6,a5,a4
        buf[i--] = digits[x % base];
    1186:	00d10a23          	sb	a3,20(sp)
    } while ((x /= base) != 0);
    118a:	12e5e863          	bltu	a1,a4,12ba <printint.constprop.0+0x1b2>
        buf[i--] = digits[x % base];
    118e:	02e876bb          	remuw	a3,a6,a4
    1192:	1682                	slli	a3,a3,0x20
    1194:	9281                	srli	a3,a3,0x20
    1196:	96b2                	add	a3,a3,a2
    1198:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    119c:	02e855bb          	divuw	a1,a6,a4
        buf[i--] = digits[x % base];
    11a0:	00d109a3          	sb	a3,19(sp)
    } while ((x /= base) != 0);
    11a4:	12e86463          	bltu	a6,a4,12cc <printint.constprop.0+0x1c4>
        buf[i--] = digits[x % base];
    11a8:	02e5f6bb          	remuw	a3,a1,a4
    11ac:	1682                	slli	a3,a3,0x20
    11ae:	9281                	srli	a3,a3,0x20
    11b0:	96b2                	add	a3,a3,a2
    11b2:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11b6:	02e5d83b          	divuw	a6,a1,a4
        buf[i--] = digits[x % base];
    11ba:	00d10923          	sb	a3,18(sp)
    } while ((x /= base) != 0);
    11be:	0ce5ec63          	bltu	a1,a4,1296 <printint.constprop.0+0x18e>
        buf[i--] = digits[x % base];
    11c2:	02e876bb          	remuw	a3,a6,a4
    11c6:	1682                	slli	a3,a3,0x20
    11c8:	9281                	srli	a3,a3,0x20
    11ca:	96b2                	add	a3,a3,a2
    11cc:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11d0:	02e855bb          	divuw	a1,a6,a4
        buf[i--] = digits[x % base];
    11d4:	00d108a3          	sb	a3,17(sp)
    } while ((x /= base) != 0);
    11d8:	10e86963          	bltu	a6,a4,12ea <printint.constprop.0+0x1e2>
        buf[i--] = digits[x % base];
    11dc:	02e5f6bb          	remuw	a3,a1,a4
    11e0:	1682                	slli	a3,a3,0x20
    11e2:	9281                	srli	a3,a3,0x20
    11e4:	96b2                	add	a3,a3,a2
    11e6:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11ea:	02e5d83b          	divuw	a6,a1,a4
        buf[i--] = digits[x % base];
    11ee:	00d10823          	sb	a3,16(sp)
    } while ((x /= base) != 0);
    11f2:	10e5e763          	bltu	a1,a4,1300 <printint.constprop.0+0x1f8>
        buf[i--] = digits[x % base];
    11f6:	02e876bb          	remuw	a3,a6,a4
    11fa:	1682                	slli	a3,a3,0x20
    11fc:	9281                	srli	a3,a3,0x20
    11fe:	96b2                	add	a3,a3,a2
    1200:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    1204:	02e857bb          	divuw	a5,a6,a4
        buf[i--] = digits[x % base];
    1208:	00d107a3          	sb	a3,15(sp)
    } while ((x /= base) != 0);
    120c:	10e86363          	bltu	a6,a4,1312 <printint.constprop.0+0x20a>
        buf[i--] = digits[x % base];
    1210:	1782                	slli	a5,a5,0x20
    1212:	9381                	srli	a5,a5,0x20
    1214:	97b2                	add	a5,a5,a2
    1216:	0007c783          	lbu	a5,0(a5)
    121a:	4599                	li	a1,6
    121c:	00f10723          	sb	a5,14(sp)

    if (sign)
    1220:	00055763          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    1224:	02d00793          	li	a5,45
    1228:	00f106a3          	sb	a5,13(sp)
        buf[i--] = digits[x % base];
    122c:	4595                	li	a1,5
    write(f, s, l);
    122e:	003c                	addi	a5,sp,8
    1230:	4641                	li	a2,16
    1232:	9e0d                	subw	a2,a2,a1
    1234:	4505                	li	a0,1
    1236:	95be                	add	a1,a1,a5
    1238:	291000ef          	jal	ra,1cc8 <write>
    i++;
    if (i < 0)
        puts("printint error");
    out(stdout, buf + i, 16 - i);
}
    123c:	70a2                	ld	ra,40(sp)
    123e:	6145                	addi	sp,sp,48
    1240:	8082                	ret
        x = -xx;
    1242:	40a0083b          	negw	a6,a0
        buf[i--] = digits[x % base];
    1246:	02b877bb          	remuw	a5,a6,a1
    124a:	00003617          	auipc	a2,0x3
    124e:	dce60613          	addi	a2,a2,-562 # 4018 <digits>
    buf[16] = 0;
    1252:	00010c23          	sb	zero,24(sp)
        buf[i--] = digits[x % base];
    1256:	0005871b          	sext.w	a4,a1
    125a:	1782                	slli	a5,a5,0x20
    125c:	9381                	srli	a5,a5,0x20
    125e:	97b2                	add	a5,a5,a2
    1260:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    1264:	02b858bb          	divuw	a7,a6,a1
        buf[i--] = digits[x % base];
    1268:	00f10ba3          	sb	a5,23(sp)
    } while ((x /= base) != 0);
    126c:	06b86963          	bltu	a6,a1,12de <printint.constprop.0+0x1d6>
        buf[i--] = digits[x % base];
    1270:	02e8f7bb          	remuw	a5,a7,a4
    1274:	1782                	slli	a5,a5,0x20
    1276:	9381                	srli	a5,a5,0x20
    1278:	97b2                	add	a5,a5,a2
    127a:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    127e:	02e8d6bb          	divuw	a3,a7,a4
        buf[i--] = digits[x % base];
    1282:	00f10b23          	sb	a5,22(sp)
    } while ((x /= base) != 0);
    1286:	ece8f8e3          	bgeu	a7,a4,1156 <printint.constprop.0+0x4e>
        buf[i--] = '-';
    128a:	02d00793          	li	a5,45
    128e:	00f10aa3          	sb	a5,21(sp)
        buf[i--] = digits[x % base];
    1292:	45b5                	li	a1,13
    1294:	bf69                	j	122e <printint.constprop.0+0x126>
    1296:	45a9                	li	a1,10
    if (sign)
    1298:	f8055be3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    129c:	02d00793          	li	a5,45
    12a0:	00f108a3          	sb	a5,17(sp)
        buf[i--] = digits[x % base];
    12a4:	45a5                	li	a1,9
    12a6:	b761                	j	122e <printint.constprop.0+0x126>
    12a8:	45b5                	li	a1,13
    if (sign)
    12aa:	f80552e3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    12ae:	02d00793          	li	a5,45
    12b2:	00f10a23          	sb	a5,20(sp)
        buf[i--] = digits[x % base];
    12b6:	45b1                	li	a1,12
    12b8:	bf9d                	j	122e <printint.constprop.0+0x126>
    12ba:	45b1                	li	a1,12
    if (sign)
    12bc:	f60559e3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    12c0:	02d00793          	li	a5,45
    12c4:	00f109a3          	sb	a5,19(sp)
        buf[i--] = digits[x % base];
    12c8:	45ad                	li	a1,11
    12ca:	b795                	j	122e <printint.constprop.0+0x126>
    12cc:	45ad                	li	a1,11
    if (sign)
    12ce:	f60550e3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    12d2:	02d00793          	li	a5,45
    12d6:	00f10923          	sb	a5,18(sp)
        buf[i--] = digits[x % base];
    12da:	45a9                	li	a1,10
    12dc:	bf89                	j	122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    12de:	02d00793          	li	a5,45
    12e2:	00f10b23          	sb	a5,22(sp)
        buf[i--] = digits[x % base];
    12e6:	45b9                	li	a1,14
    12e8:	b799                	j	122e <printint.constprop.0+0x126>
    12ea:	45a5                	li	a1,9
    if (sign)
    12ec:	f40551e3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    12f0:	02d00793          	li	a5,45
    12f4:	00f10823          	sb	a5,16(sp)
        buf[i--] = digits[x % base];
    12f8:	45a1                	li	a1,8
    12fa:	bf15                	j	122e <printint.constprop.0+0x126>
    i = 15;
    12fc:	45bd                	li	a1,15
    12fe:	bf05                	j	122e <printint.constprop.0+0x126>
        buf[i--] = digits[x % base];
    1300:	45a1                	li	a1,8
    if (sign)
    1302:	f20556e3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    1306:	02d00793          	li	a5,45
    130a:	00f107a3          	sb	a5,15(sp)
        buf[i--] = digits[x % base];
    130e:	459d                	li	a1,7
    1310:	bf39                	j	122e <printint.constprop.0+0x126>
    1312:	459d                	li	a1,7
    if (sign)
    1314:	f0055de3          	bgez	a0,122e <printint.constprop.0+0x126>
        buf[i--] = '-';
    1318:	02d00793          	li	a5,45
    131c:	00f10723          	sb	a5,14(sp)
        buf[i--] = digits[x % base];
    1320:	4599                	li	a1,6
    1322:	b731                	j	122e <printint.constprop.0+0x126>

0000000000001324 <getchar>:
{
    1324:	1101                	addi	sp,sp,-32
    read(stdin, &byte, 1);
    1326:	00f10593          	addi	a1,sp,15
    132a:	4605                	li	a2,1
    132c:	4501                	li	a0,0
{
    132e:	ec06                	sd	ra,24(sp)
    char byte = 0;
    1330:	000107a3          	sb	zero,15(sp)
    read(stdin, &byte, 1);
    1334:	18b000ef          	jal	ra,1cbe <read>
}
    1338:	60e2                	ld	ra,24(sp)
    133a:	00f14503          	lbu	a0,15(sp)
    133e:	6105                	addi	sp,sp,32
    1340:	8082                	ret

0000000000001342 <putchar>:
{
    1342:	1101                	addi	sp,sp,-32
    1344:	87aa                	mv	a5,a0
    return write(stdout, &byte, 1);
    1346:	00f10593          	addi	a1,sp,15
    134a:	4605                	li	a2,1
    134c:	4505                	li	a0,1
{
    134e:	ec06                	sd	ra,24(sp)
    char byte = c;
    1350:	00f107a3          	sb	a5,15(sp)
    return write(stdout, &byte, 1);
    1354:	175000ef          	jal	ra,1cc8 <write>
}
    1358:	60e2                	ld	ra,24(sp)
    135a:	2501                	sext.w	a0,a0
    135c:	6105                	addi	sp,sp,32
    135e:	8082                	ret

0000000000001360 <puts>:
{
    1360:	1141                	addi	sp,sp,-16
    1362:	e406                	sd	ra,8(sp)
    1364:	e022                	sd	s0,0(sp)
    1366:	842a                	mv	s0,a0
    r = -(write(stdout, s, strlen(s)) < 0);
    1368:	57c000ef          	jal	ra,18e4 <strlen>
    136c:	862a                	mv	a2,a0
    136e:	85a2                	mv	a1,s0
    1370:	4505                	li	a0,1
    1372:	157000ef          	jal	ra,1cc8 <write>
}
    1376:	60a2                	ld	ra,8(sp)
    1378:	6402                	ld	s0,0(sp)
    r = -(write(stdout, s, strlen(s)) < 0);
    137a:	957d                	srai	a0,a0,0x3f
    return r;
    137c:	2501                	sext.w	a0,a0
}
    137e:	0141                	addi	sp,sp,16
    1380:	8082                	ret

0000000000001382 <printf>:
    out(stdout, buf, i);
}

// Print to the console. only understands %d, %x, %p, %s.
void printf(const char *fmt, ...)
{
    1382:	7171                	addi	sp,sp,-176
    1384:	fc56                	sd	s5,56(sp)
    1386:	ed3e                	sd	a5,152(sp)
    buf[i++] = '0';
    1388:	7ae1                	lui	s5,0xffff8
    va_list ap;
    int cnt = 0, l = 0;
    char *a, *z, *s = (char *)fmt, str;
    int f = stdout;

    va_start(ap, fmt);
    138a:	18bc                	addi	a5,sp,120
{
    138c:	e8ca                	sd	s2,80(sp)
    138e:	e4ce                	sd	s3,72(sp)
    1390:	e0d2                	sd	s4,64(sp)
    1392:	f85a                	sd	s6,48(sp)
    1394:	f486                	sd	ra,104(sp)
    1396:	f0a2                	sd	s0,96(sp)
    1398:	eca6                	sd	s1,88(sp)
    139a:	fcae                	sd	a1,120(sp)
    139c:	e132                	sd	a2,128(sp)
    139e:	e536                	sd	a3,136(sp)
    13a0:	e93a                	sd	a4,144(sp)
    13a2:	f142                	sd	a6,160(sp)
    13a4:	f546                	sd	a7,168(sp)
    va_start(ap, fmt);
    13a6:	e03e                	sd	a5,0(sp)
    for (;;)
    {
        if (!*s)
            break;
        for (a = s; *s && *s != '%'; s++)
    13a8:	02500913          	li	s2,37
        out(f, a, l);
        if (l)
            continue;
        if (s[1] == 0)
            break;
        switch (s[1])
    13ac:	07300a13          	li	s4,115
        case 'p':
            printptr(va_arg(ap, uint64));
            break;
        case 's':
            if ((a = va_arg(ap, char *)) == 0)
                a = "(null)";
    13b0:	00001b17          	auipc	s6,0x1
    13b4:	c30b0b13          	addi	s6,s6,-976 # 1fe0 <__clone+0xe0>
    buf[i++] = '0';
    13b8:	830aca93          	xori	s5,s5,-2000
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    13bc:	00003997          	auipc	s3,0x3
    13c0:	c5c98993          	addi	s3,s3,-932 # 4018 <digits>
        if (!*s)
    13c4:	00054783          	lbu	a5,0(a0)
    13c8:	16078a63          	beqz	a5,153c <printf+0x1ba>
    13cc:	862a                	mv	a2,a0
        for (a = s; *s && *s != '%'; s++)
    13ce:	19278163          	beq	a5,s2,1550 <printf+0x1ce>
    13d2:	00164783          	lbu	a5,1(a2)
    13d6:	0605                	addi	a2,a2,1
    13d8:	fbfd                	bnez	a5,13ce <printf+0x4c>
    13da:	84b2                	mv	s1,a2
        l = z - a;
    13dc:	40a6043b          	subw	s0,a2,a0
    write(f, s, l);
    13e0:	85aa                	mv	a1,a0
    13e2:	8622                	mv	a2,s0
    13e4:	4505                	li	a0,1
    13e6:	0e3000ef          	jal	ra,1cc8 <write>
        if (l)
    13ea:	18041c63          	bnez	s0,1582 <printf+0x200>
        if (s[1] == 0)
    13ee:	0014c783          	lbu	a5,1(s1)
    13f2:	14078563          	beqz	a5,153c <printf+0x1ba>
        switch (s[1])
    13f6:	1d478063          	beq	a5,s4,15b6 <printf+0x234>
    13fa:	18fa6663          	bltu	s4,a5,1586 <printf+0x204>
    13fe:	06400713          	li	a4,100
    1402:	1ae78063          	beq	a5,a4,15a2 <printf+0x220>
    1406:	07000713          	li	a4,112
    140a:	1ce79963          	bne	a5,a4,15dc <printf+0x25a>
            printptr(va_arg(ap, uint64));
    140e:	6702                	ld	a4,0(sp)
    buf[i++] = '0';
    1410:	01511423          	sh	s5,8(sp)
    write(f, s, l);
    1414:	4649                	li	a2,18
            printptr(va_arg(ap, uint64));
    1416:	631c                	ld	a5,0(a4)
    1418:	0721                	addi	a4,a4,8
    141a:	e03a                	sd	a4,0(sp)
    for (j = 0; j < (sizeof(uint64) * 2); j++, x <<= 4)
    141c:	00479293          	slli	t0,a5,0x4
    1420:	00879f93          	slli	t6,a5,0x8
    1424:	00c79f13          	slli	t5,a5,0xc
    1428:	01079e93          	slli	t4,a5,0x10
    142c:	01479e13          	slli	t3,a5,0x14
    1430:	01879313          	slli	t1,a5,0x18
    1434:	01c79893          	slli	a7,a5,0x1c
    1438:	02479813          	slli	a6,a5,0x24
    143c:	02879513          	slli	a0,a5,0x28
    1440:	02c79593          	slli	a1,a5,0x2c
    1444:	03079693          	slli	a3,a5,0x30
    1448:	03479713          	slli	a4,a5,0x34
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    144c:	03c7d413          	srli	s0,a5,0x3c
    1450:	01c7d39b          	srliw	t2,a5,0x1c
    1454:	03c2d293          	srli	t0,t0,0x3c
    1458:	03cfdf93          	srli	t6,t6,0x3c
    145c:	03cf5f13          	srli	t5,t5,0x3c
    1460:	03cede93          	srli	t4,t4,0x3c
    1464:	03ce5e13          	srli	t3,t3,0x3c
    1468:	03c35313          	srli	t1,t1,0x3c
    146c:	03c8d893          	srli	a7,a7,0x3c
    1470:	03c85813          	srli	a6,a6,0x3c
    1474:	9171                	srli	a0,a0,0x3c
    1476:	91f1                	srli	a1,a1,0x3c
    1478:	92f1                	srli	a3,a3,0x3c
    147a:	9371                	srli	a4,a4,0x3c
    147c:	96ce                	add	a3,a3,s3
    147e:	974e                	add	a4,a4,s3
    1480:	944e                	add	s0,s0,s3
    1482:	92ce                	add	t0,t0,s3
    1484:	9fce                	add	t6,t6,s3
    1486:	9f4e                	add	t5,t5,s3
    1488:	9ece                	add	t4,t4,s3
    148a:	9e4e                	add	t3,t3,s3
    148c:	934e                	add	t1,t1,s3
    148e:	98ce                	add	a7,a7,s3
    1490:	93ce                	add	t2,t2,s3
    1492:	984e                	add	a6,a6,s3
    1494:	954e                	add	a0,a0,s3
    1496:	95ce                	add	a1,a1,s3
    1498:	0006c083          	lbu	ra,0(a3)
    149c:	0002c283          	lbu	t0,0(t0)
    14a0:	00074683          	lbu	a3,0(a4)
    14a4:	000fcf83          	lbu	t6,0(t6)
    14a8:	000f4f03          	lbu	t5,0(t5)
    14ac:	000ece83          	lbu	t4,0(t4)
    14b0:	000e4e03          	lbu	t3,0(t3)
    14b4:	00034303          	lbu	t1,0(t1)
    14b8:	0008c883          	lbu	a7,0(a7)
    14bc:	0003c383          	lbu	t2,0(t2)
    14c0:	00084803          	lbu	a6,0(a6)
    14c4:	00054503          	lbu	a0,0(a0)
    14c8:	0005c583          	lbu	a1,0(a1)
    14cc:	00044403          	lbu	s0,0(s0)
    for (j = 0; j < (sizeof(uint64) * 2); j++, x <<= 4)
    14d0:	03879713          	slli	a4,a5,0x38
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    14d4:	9371                	srli	a4,a4,0x3c
    14d6:	8bbd                	andi	a5,a5,15
    14d8:	974e                	add	a4,a4,s3
    14da:	97ce                	add	a5,a5,s3
    14dc:	005105a3          	sb	t0,11(sp)
    14e0:	01f10623          	sb	t6,12(sp)
    14e4:	01e106a3          	sb	t5,13(sp)
    14e8:	01d10723          	sb	t4,14(sp)
    14ec:	01c107a3          	sb	t3,15(sp)
    14f0:	00610823          	sb	t1,16(sp)
    14f4:	011108a3          	sb	a7,17(sp)
    14f8:	00710923          	sb	t2,18(sp)
    14fc:	010109a3          	sb	a6,19(sp)
    1500:	00a10a23          	sb	a0,20(sp)
    1504:	00b10aa3          	sb	a1,21(sp)
    1508:	00110b23          	sb	ra,22(sp)
    150c:	00d10ba3          	sb	a3,23(sp)
    1510:	00810523          	sb	s0,10(sp)
    1514:	00074703          	lbu	a4,0(a4)
    1518:	0007c783          	lbu	a5,0(a5)
    write(f, s, l);
    151c:	002c                	addi	a1,sp,8
    151e:	4505                	li	a0,1
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    1520:	00e10c23          	sb	a4,24(sp)
    1524:	00f10ca3          	sb	a5,25(sp)
    buf[i] = 0;
    1528:	00010d23          	sb	zero,26(sp)
    write(f, s, l);
    152c:	79c000ef          	jal	ra,1cc8 <write>
            // Print unknown % sequence to draw attention.
            putchar('%');
            putchar(s[1]);
            break;
        }
        s += 2;
    1530:	00248513          	addi	a0,s1,2
        if (!*s)
    1534:	00054783          	lbu	a5,0(a0)
    1538:	e8079ae3          	bnez	a5,13cc <printf+0x4a>
    }
    va_end(ap);
}
    153c:	70a6                	ld	ra,104(sp)
    153e:	7406                	ld	s0,96(sp)
    1540:	64e6                	ld	s1,88(sp)
    1542:	6946                	ld	s2,80(sp)
    1544:	69a6                	ld	s3,72(sp)
    1546:	6a06                	ld	s4,64(sp)
    1548:	7ae2                	ld	s5,56(sp)
    154a:	7b42                	ld	s6,48(sp)
    154c:	614d                	addi	sp,sp,176
    154e:	8082                	ret
        for (z = s; s[0] == '%' && s[1] == '%'; z++, s += 2)
    1550:	00064783          	lbu	a5,0(a2)
    1554:	84b2                	mv	s1,a2
    1556:	01278963          	beq	a5,s2,1568 <printf+0x1e6>
    155a:	b549                	j	13dc <printf+0x5a>
    155c:	0024c783          	lbu	a5,2(s1)
    1560:	0605                	addi	a2,a2,1
    1562:	0489                	addi	s1,s1,2
    1564:	e7279ce3          	bne	a5,s2,13dc <printf+0x5a>
    1568:	0014c783          	lbu	a5,1(s1)
    156c:	ff2788e3          	beq	a5,s2,155c <printf+0x1da>
        l = z - a;
    1570:	40a6043b          	subw	s0,a2,a0
    write(f, s, l);
    1574:	85aa                	mv	a1,a0
    1576:	8622                	mv	a2,s0
    1578:	4505                	li	a0,1
    157a:	74e000ef          	jal	ra,1cc8 <write>
        if (l)
    157e:	e60408e3          	beqz	s0,13ee <printf+0x6c>
    1582:	8526                	mv	a0,s1
    1584:	b581                	j	13c4 <printf+0x42>
        switch (s[1])
    1586:	07800713          	li	a4,120
    158a:	04e79963          	bne	a5,a4,15dc <printf+0x25a>
            printint(va_arg(ap, int), 16, 1);
    158e:	6782                	ld	a5,0(sp)
    1590:	45c1                	li	a1,16
    1592:	4388                	lw	a0,0(a5)
    1594:	07a1                	addi	a5,a5,8
    1596:	e03e                	sd	a5,0(sp)
    1598:	b71ff0ef          	jal	ra,1108 <printint.constprop.0>
        s += 2;
    159c:	00248513          	addi	a0,s1,2
    15a0:	bf51                	j	1534 <printf+0x1b2>
            printint(va_arg(ap, int), 10, 1);
    15a2:	6782                	ld	a5,0(sp)
    15a4:	45a9                	li	a1,10
    15a6:	4388                	lw	a0,0(a5)
    15a8:	07a1                	addi	a5,a5,8
    15aa:	e03e                	sd	a5,0(sp)
    15ac:	b5dff0ef          	jal	ra,1108 <printint.constprop.0>
        s += 2;
    15b0:	00248513          	addi	a0,s1,2
    15b4:	b741                	j	1534 <printf+0x1b2>
            if ((a = va_arg(ap, char *)) == 0)
    15b6:	6782                	ld	a5,0(sp)
    15b8:	6380                	ld	s0,0(a5)
    15ba:	07a1                	addi	a5,a5,8
    15bc:	e03e                	sd	a5,0(sp)
    15be:	c031                	beqz	s0,1602 <printf+0x280>
            l = strnlen(a, 200);
    15c0:	0c800593          	li	a1,200
    15c4:	8522                	mv	a0,s0
    15c6:	40a000ef          	jal	ra,19d0 <strnlen>
    write(f, s, l);
    15ca:	0005061b          	sext.w	a2,a0
    15ce:	85a2                	mv	a1,s0
    15d0:	4505                	li	a0,1
    15d2:	6f6000ef          	jal	ra,1cc8 <write>
        s += 2;
    15d6:	00248513          	addi	a0,s1,2
    15da:	bfa9                	j	1534 <printf+0x1b2>
    return write(stdout, &byte, 1);
    15dc:	4605                	li	a2,1
    15de:	002c                	addi	a1,sp,8
    15e0:	4505                	li	a0,1
    char byte = c;
    15e2:	01210423          	sb	s2,8(sp)
    return write(stdout, &byte, 1);
    15e6:	6e2000ef          	jal	ra,1cc8 <write>
    char byte = c;
    15ea:	0014c783          	lbu	a5,1(s1)
    return write(stdout, &byte, 1);
    15ee:	4605                	li	a2,1
    15f0:	002c                	addi	a1,sp,8
    15f2:	4505                	li	a0,1
    char byte = c;
    15f4:	00f10423          	sb	a5,8(sp)
    return write(stdout, &byte, 1);
    15f8:	6d0000ef          	jal	ra,1cc8 <write>
        s += 2;
    15fc:	00248513          	addi	a0,s1,2
    1600:	bf15                	j	1534 <printf+0x1b2>
                a = "(null)";
    1602:	845a                	mv	s0,s6
    1604:	bf75                	j	15c0 <printf+0x23e>

0000000000001606 <panic>:
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

void panic(char *m)
{
    1606:	1141                	addi	sp,sp,-16
    1608:	e406                	sd	ra,8(sp)
    puts(m);
    160a:	d57ff0ef          	jal	ra,1360 <puts>
    exit(-100);
}
    160e:	60a2                	ld	ra,8(sp)
    exit(-100);
    1610:	f9c00513          	li	a0,-100
}
    1614:	0141                	addi	sp,sp,16
    exit(-100);
    1616:	a709                	j	1d18 <exit>

0000000000001618 <isspace>:
#define HIGHS (ONES * (UCHAR_MAX / 2 + 1))
#define HASZERO(x) (((x)-ONES) & ~(x)&HIGHS)

int isspace(int c)
{
    return c == ' ' || (unsigned)c - '\t' < 5;
    1618:	02000793          	li	a5,32
    161c:	00f50663          	beq	a0,a5,1628 <isspace+0x10>
    1620:	355d                	addiw	a0,a0,-9
    1622:	00553513          	sltiu	a0,a0,5
    1626:	8082                	ret
    1628:	4505                	li	a0,1
}
    162a:	8082                	ret

000000000000162c <isdigit>:

int isdigit(int c)
{
    return (unsigned)c - '0' < 10;
    162c:	fd05051b          	addiw	a0,a0,-48
}
    1630:	00a53513          	sltiu	a0,a0,10
    1634:	8082                	ret

0000000000001636 <atoi>:
    return c == ' ' || (unsigned)c - '\t' < 5;
    1636:	02000613          	li	a2,32
    163a:	4591                	li	a1,4

int atoi(const char *s)
{
    int n = 0, neg = 0;
    while (isspace(*s))
    163c:	00054703          	lbu	a4,0(a0)
    return c == ' ' || (unsigned)c - '\t' < 5;
    1640:	ff77069b          	addiw	a3,a4,-9
    1644:	04c70d63          	beq	a4,a2,169e <atoi+0x68>
    1648:	0007079b          	sext.w	a5,a4
    164c:	04d5f963          	bgeu	a1,a3,169e <atoi+0x68>
        s++;
    switch (*s)
    1650:	02b00693          	li	a3,43
    1654:	04d70a63          	beq	a4,a3,16a8 <atoi+0x72>
    1658:	02d00693          	li	a3,45
    165c:	06d70463          	beq	a4,a3,16c4 <atoi+0x8e>
        neg = 1;
    case '+':
        s++;
    }
    /* Compute n as a negative number to avoid overflow on INT_MIN */
    while (isdigit(*s))
    1660:	fd07859b          	addiw	a1,a5,-48
    1664:	4625                	li	a2,9
    1666:	873e                	mv	a4,a5
    1668:	86aa                	mv	a3,a0
    int n = 0, neg = 0;
    166a:	4e01                	li	t3,0
    while (isdigit(*s))
    166c:	04b66a63          	bltu	a2,a1,16c0 <atoi+0x8a>
    int n = 0, neg = 0;
    1670:	4501                	li	a0,0
    while (isdigit(*s))
    1672:	4825                	li	a6,9
    1674:	0016c603          	lbu	a2,1(a3)
        n = 10 * n - (*s++ - '0');
    1678:	0025179b          	slliw	a5,a0,0x2
    167c:	9d3d                	addw	a0,a0,a5
    167e:	fd07031b          	addiw	t1,a4,-48
    1682:	0015189b          	slliw	a7,a0,0x1
    while (isdigit(*s))
    1686:	fd06059b          	addiw	a1,a2,-48
        n = 10 * n - (*s++ - '0');
    168a:	0685                	addi	a3,a3,1
    168c:	4068853b          	subw	a0,a7,t1
    while (isdigit(*s))
    1690:	0006071b          	sext.w	a4,a2
    1694:	feb870e3          	bgeu	a6,a1,1674 <atoi+0x3e>
    return neg ? n : -n;
    1698:	000e0563          	beqz	t3,16a2 <atoi+0x6c>
}
    169c:	8082                	ret
        s++;
    169e:	0505                	addi	a0,a0,1
    16a0:	bf71                	j	163c <atoi+0x6>
    return neg ? n : -n;
    16a2:	4113053b          	subw	a0,t1,a7
    16a6:	8082                	ret
    while (isdigit(*s))
    16a8:	00154783          	lbu	a5,1(a0)
    16ac:	4625                	li	a2,9
        s++;
    16ae:	00150693          	addi	a3,a0,1
    while (isdigit(*s))
    16b2:	fd07859b          	addiw	a1,a5,-48
    16b6:	0007871b          	sext.w	a4,a5
    int n = 0, neg = 0;
    16ba:	4e01                	li	t3,0
    while (isdigit(*s))
    16bc:	fab67ae3          	bgeu	a2,a1,1670 <atoi+0x3a>
    16c0:	4501                	li	a0,0
}
    16c2:	8082                	ret
    while (isdigit(*s))
    16c4:	00154783          	lbu	a5,1(a0)
    16c8:	4625                	li	a2,9
        s++;
    16ca:	00150693          	addi	a3,a0,1
    while (isdigit(*s))
    16ce:	fd07859b          	addiw	a1,a5,-48
    16d2:	0007871b          	sext.w	a4,a5
    16d6:	feb665e3          	bltu	a2,a1,16c0 <atoi+0x8a>
        neg = 1;
    16da:	4e05                	li	t3,1
    16dc:	bf51                	j	1670 <atoi+0x3a>

00000000000016de <memset>:

void *memset(void *dest, int c, size_t n)
{
    char *p = dest;
    for (int i = 0; i < n; ++i, *(p++) = c)
    16de:	16060d63          	beqz	a2,1858 <memset+0x17a>
    16e2:	40a007b3          	neg	a5,a0
    16e6:	8b9d                	andi	a5,a5,7
    16e8:	00778713          	addi	a4,a5,7
    16ec:	482d                	li	a6,11
    16ee:	0ff5f593          	zext.b	a1,a1
    16f2:	fff60693          	addi	a3,a2,-1
    16f6:	17076263          	bltu	a4,a6,185a <memset+0x17c>
    16fa:	16e6ea63          	bltu	a3,a4,186e <memset+0x190>
    16fe:	16078563          	beqz	a5,1868 <memset+0x18a>
    1702:	00b50023          	sb	a1,0(a0)
    1706:	4705                	li	a4,1
    1708:	00150e93          	addi	t4,a0,1
    170c:	14e78c63          	beq	a5,a4,1864 <memset+0x186>
    1710:	00b500a3          	sb	a1,1(a0)
    1714:	4709                	li	a4,2
    1716:	00250e93          	addi	t4,a0,2
    171a:	14e78d63          	beq	a5,a4,1874 <memset+0x196>
    171e:	00b50123          	sb	a1,2(a0)
    1722:	470d                	li	a4,3
    1724:	00350e93          	addi	t4,a0,3
    1728:	12e78b63          	beq	a5,a4,185e <memset+0x180>
    172c:	00b501a3          	sb	a1,3(a0)
    1730:	4711                	li	a4,4
    1732:	00450e93          	addi	t4,a0,4
    1736:	14e78163          	beq	a5,a4,1878 <memset+0x19a>
    173a:	00b50223          	sb	a1,4(a0)
    173e:	4715                	li	a4,5
    1740:	00550e93          	addi	t4,a0,5
    1744:	12e78c63          	beq	a5,a4,187c <memset+0x19e>
    1748:	00b502a3          	sb	a1,5(a0)
    174c:	471d                	li	a4,7
    174e:	00650e93          	addi	t4,a0,6
    1752:	12e79763          	bne	a5,a4,1880 <memset+0x1a2>
    1756:	00750e93          	addi	t4,a0,7
    175a:	00b50323          	sb	a1,6(a0)
    175e:	4f1d                	li	t5,7
    1760:	00859713          	slli	a4,a1,0x8
    1764:	8f4d                	or	a4,a4,a1
    1766:	01059e13          	slli	t3,a1,0x10
    176a:	01c76e33          	or	t3,a4,t3
    176e:	01859313          	slli	t1,a1,0x18
    1772:	006e6333          	or	t1,t3,t1
    1776:	02059893          	slli	a7,a1,0x20
    177a:	011368b3          	or	a7,t1,a7
    177e:	02859813          	slli	a6,a1,0x28
    1782:	40f60333          	sub	t1,a2,a5
    1786:	0108e833          	or	a6,a7,a6
    178a:	03059693          	slli	a3,a1,0x30
    178e:	00d866b3          	or	a3,a6,a3
    1792:	03859713          	slli	a4,a1,0x38
    1796:	97aa                	add	a5,a5,a0
    1798:	ff837813          	andi	a6,t1,-8
    179c:	8f55                	or	a4,a4,a3
    179e:	00f806b3          	add	a3,a6,a5
    17a2:	e398                	sd	a4,0(a5)
    17a4:	07a1                	addi	a5,a5,8
    17a6:	fed79ee3          	bne	a5,a3,17a2 <memset+0xc4>
    17aa:	ff837693          	andi	a3,t1,-8
    17ae:	00de87b3          	add	a5,t4,a3
    17b2:	01e6873b          	addw	a4,a3,t5
    17b6:	0ad30663          	beq	t1,a3,1862 <memset+0x184>
    17ba:	00b78023          	sb	a1,0(a5)
    17be:	0017069b          	addiw	a3,a4,1
    17c2:	08c6fb63          	bgeu	a3,a2,1858 <memset+0x17a>
    17c6:	00b780a3          	sb	a1,1(a5)
    17ca:	0027069b          	addiw	a3,a4,2
    17ce:	08c6f563          	bgeu	a3,a2,1858 <memset+0x17a>
    17d2:	00b78123          	sb	a1,2(a5)
    17d6:	0037069b          	addiw	a3,a4,3
    17da:	06c6ff63          	bgeu	a3,a2,1858 <memset+0x17a>
    17de:	00b781a3          	sb	a1,3(a5)
    17e2:	0047069b          	addiw	a3,a4,4
    17e6:	06c6f963          	bgeu	a3,a2,1858 <memset+0x17a>
    17ea:	00b78223          	sb	a1,4(a5)
    17ee:	0057069b          	addiw	a3,a4,5
    17f2:	06c6f363          	bgeu	a3,a2,1858 <memset+0x17a>
    17f6:	00b782a3          	sb	a1,5(a5)
    17fa:	0067069b          	addiw	a3,a4,6
    17fe:	04c6fd63          	bgeu	a3,a2,1858 <memset+0x17a>
    1802:	00b78323          	sb	a1,6(a5)
    1806:	0077069b          	addiw	a3,a4,7
    180a:	04c6f763          	bgeu	a3,a2,1858 <memset+0x17a>
    180e:	00b783a3          	sb	a1,7(a5)
    1812:	0087069b          	addiw	a3,a4,8
    1816:	04c6f163          	bgeu	a3,a2,1858 <memset+0x17a>
    181a:	00b78423          	sb	a1,8(a5)
    181e:	0097069b          	addiw	a3,a4,9
    1822:	02c6fb63          	bgeu	a3,a2,1858 <memset+0x17a>
    1826:	00b784a3          	sb	a1,9(a5)
    182a:	00a7069b          	addiw	a3,a4,10
    182e:	02c6f563          	bgeu	a3,a2,1858 <memset+0x17a>
    1832:	00b78523          	sb	a1,10(a5)
    1836:	00b7069b          	addiw	a3,a4,11
    183a:	00c6ff63          	bgeu	a3,a2,1858 <memset+0x17a>
    183e:	00b785a3          	sb	a1,11(a5)
    1842:	00c7069b          	addiw	a3,a4,12
    1846:	00c6f963          	bgeu	a3,a2,1858 <memset+0x17a>
    184a:	00b78623          	sb	a1,12(a5)
    184e:	2735                	addiw	a4,a4,13
    1850:	00c77463          	bgeu	a4,a2,1858 <memset+0x17a>
    1854:	00b786a3          	sb	a1,13(a5)
        ;
    return dest;
}
    1858:	8082                	ret
    185a:	472d                	li	a4,11
    185c:	bd79                	j	16fa <memset+0x1c>
    for (int i = 0; i < n; ++i, *(p++) = c)
    185e:	4f0d                	li	t5,3
    1860:	b701                	j	1760 <memset+0x82>
    1862:	8082                	ret
    1864:	4f05                	li	t5,1
    1866:	bded                	j	1760 <memset+0x82>
    1868:	8eaa                	mv	t4,a0
    186a:	4f01                	li	t5,0
    186c:	bdd5                	j	1760 <memset+0x82>
    186e:	87aa                	mv	a5,a0
    1870:	4701                	li	a4,0
    1872:	b7a1                	j	17ba <memset+0xdc>
    1874:	4f09                	li	t5,2
    1876:	b5ed                	j	1760 <memset+0x82>
    1878:	4f11                	li	t5,4
    187a:	b5dd                	j	1760 <memset+0x82>
    187c:	4f15                	li	t5,5
    187e:	b5cd                	j	1760 <memset+0x82>
    1880:	4f19                	li	t5,6
    1882:	bdf9                	j	1760 <memset+0x82>

0000000000001884 <strcmp>:

int strcmp(const char *l, const char *r)
{
    for (; *l == *r && *l; l++, r++)
    1884:	00054783          	lbu	a5,0(a0)
    1888:	0005c703          	lbu	a4,0(a1)
    188c:	00e79863          	bne	a5,a4,189c <strcmp+0x18>
    1890:	0505                	addi	a0,a0,1
    1892:	0585                	addi	a1,a1,1
    1894:	fbe5                	bnez	a5,1884 <strcmp>
    1896:	4501                	li	a0,0
        ;
    return *(unsigned char *)l - *(unsigned char *)r;
}
    1898:	9d19                	subw	a0,a0,a4
    189a:	8082                	ret
    return *(unsigned char *)l - *(unsigned char *)r;
    189c:	0007851b          	sext.w	a0,a5
    18a0:	bfe5                	j	1898 <strcmp+0x14>

00000000000018a2 <strncmp>:

int strncmp(const char *_l, const char *_r, size_t n)
{
    const unsigned char *l = (void *)_l, *r = (void *)_r;
    if (!n--)
    18a2:	ce05                	beqz	a2,18da <strncmp+0x38>
        return 0;
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    18a4:	00054703          	lbu	a4,0(a0)
    18a8:	0005c783          	lbu	a5,0(a1)
    18ac:	cb0d                	beqz	a4,18de <strncmp+0x3c>
    if (!n--)
    18ae:	167d                	addi	a2,a2,-1
    18b0:	00c506b3          	add	a3,a0,a2
    18b4:	a819                	j	18ca <strncmp+0x28>
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    18b6:	00a68e63          	beq	a3,a0,18d2 <strncmp+0x30>
    18ba:	0505                	addi	a0,a0,1
    18bc:	00e79b63          	bne	a5,a4,18d2 <strncmp+0x30>
    18c0:	00054703          	lbu	a4,0(a0)
        ;
    return *l - *r;
    18c4:	0005c783          	lbu	a5,0(a1)
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    18c8:	cb19                	beqz	a4,18de <strncmp+0x3c>
    18ca:	0005c783          	lbu	a5,0(a1)
    18ce:	0585                	addi	a1,a1,1
    18d0:	f3fd                	bnez	a5,18b6 <strncmp+0x14>
    return *l - *r;
    18d2:	0007051b          	sext.w	a0,a4
    18d6:	9d1d                	subw	a0,a0,a5
    18d8:	8082                	ret
        return 0;
    18da:	4501                	li	a0,0
}
    18dc:	8082                	ret
    18de:	4501                	li	a0,0
    return *l - *r;
    18e0:	9d1d                	subw	a0,a0,a5
    18e2:	8082                	ret

00000000000018e4 <strlen>:
size_t strlen(const char *s)
{
    const char *a = s;
    typedef size_t __attribute__((__may_alias__)) word;
    const word *w;
    for (; (uintptr_t)s % SS; s++)
    18e4:	00757793          	andi	a5,a0,7
    18e8:	cf89                	beqz	a5,1902 <strlen+0x1e>
    18ea:	87aa                	mv	a5,a0
    18ec:	a029                	j	18f6 <strlen+0x12>
    18ee:	0785                	addi	a5,a5,1
    18f0:	0077f713          	andi	a4,a5,7
    18f4:	cb01                	beqz	a4,1904 <strlen+0x20>
        if (!*s)
    18f6:	0007c703          	lbu	a4,0(a5)
    18fa:	fb75                	bnez	a4,18ee <strlen+0xa>
    for (w = (const void *)s; !HASZERO(*w); w++)
        ;
    s = (const void *)w;
    for (; *s; s++)
        ;
    return s - a;
    18fc:	40a78533          	sub	a0,a5,a0
}
    1900:	8082                	ret
    for (; (uintptr_t)s % SS; s++)
    1902:	87aa                	mv	a5,a0
    for (w = (const void *)s; !HASZERO(*w); w++)
    1904:	6394                	ld	a3,0(a5)
    1906:	00000597          	auipc	a1,0x0
    190a:	6e25b583          	ld	a1,1762(a1) # 1fe8 <__clone+0xe8>
    190e:	00000617          	auipc	a2,0x0
    1912:	6e263603          	ld	a2,1762(a2) # 1ff0 <__clone+0xf0>
    1916:	a019                	j	191c <strlen+0x38>
    1918:	6794                	ld	a3,8(a5)
    191a:	07a1                	addi	a5,a5,8
    191c:	00b68733          	add	a4,a3,a1
    1920:	fff6c693          	not	a3,a3
    1924:	8f75                	and	a4,a4,a3
    1926:	8f71                	and	a4,a4,a2
    1928:	db65                	beqz	a4,1918 <strlen+0x34>
    for (; *s; s++)
    192a:	0007c703          	lbu	a4,0(a5)
    192e:	d779                	beqz	a4,18fc <strlen+0x18>
    1930:	0017c703          	lbu	a4,1(a5)
    1934:	0785                	addi	a5,a5,1
    1936:	d379                	beqz	a4,18fc <strlen+0x18>
    1938:	0017c703          	lbu	a4,1(a5)
    193c:	0785                	addi	a5,a5,1
    193e:	fb6d                	bnez	a4,1930 <strlen+0x4c>
    1940:	bf75                	j	18fc <strlen+0x18>

0000000000001942 <memchr>:

void *memchr(const void *src, int c, size_t n)
{
    const unsigned char *s = src;
    c = (unsigned char)c;
    for (; ((uintptr_t)s & ALIGN) && n && *s != c; s++, n--)
    1942:	00757713          	andi	a4,a0,7
{
    1946:	87aa                	mv	a5,a0
    c = (unsigned char)c;
    1948:	0ff5f593          	zext.b	a1,a1
    for (; ((uintptr_t)s & ALIGN) && n && *s != c; s++, n--)
    194c:	cb19                	beqz	a4,1962 <memchr+0x20>
    194e:	ce25                	beqz	a2,19c6 <memchr+0x84>
    1950:	0007c703          	lbu	a4,0(a5)
    1954:	04b70e63          	beq	a4,a1,19b0 <memchr+0x6e>
    1958:	0785                	addi	a5,a5,1
    195a:	0077f713          	andi	a4,a5,7
    195e:	167d                	addi	a2,a2,-1
    1960:	f77d                	bnez	a4,194e <memchr+0xc>
            ;
        s = (const void *)w;
    }
    for (; n && *s != c; s++, n--)
        ;
    return n ? (void *)s : 0;
    1962:	4501                	li	a0,0
    if (n && *s != c)
    1964:	c235                	beqz	a2,19c8 <memchr+0x86>
    1966:	0007c703          	lbu	a4,0(a5)
    196a:	04b70363          	beq	a4,a1,19b0 <memchr+0x6e>
        size_t k = ONES * c;
    196e:	00000517          	auipc	a0,0x0
    1972:	68a53503          	ld	a0,1674(a0) # 1ff8 <__clone+0xf8>
        for (w = (const void *)s; n >= SS && !HASZERO(*w ^ k); w++, n -= SS)
    1976:	471d                	li	a4,7
        size_t k = ONES * c;
    1978:	02a58533          	mul	a0,a1,a0
        for (w = (const void *)s; n >= SS && !HASZERO(*w ^ k); w++, n -= SS)
    197c:	02c77a63          	bgeu	a4,a2,19b0 <memchr+0x6e>
    1980:	00000897          	auipc	a7,0x0
    1984:	6688b883          	ld	a7,1640(a7) # 1fe8 <__clone+0xe8>
    1988:	00000817          	auipc	a6,0x0
    198c:	66883803          	ld	a6,1640(a6) # 1ff0 <__clone+0xf0>
    1990:	431d                	li	t1,7
    1992:	a029                	j	199c <memchr+0x5a>
    1994:	1661                	addi	a2,a2,-8
    1996:	07a1                	addi	a5,a5,8
    1998:	02c37963          	bgeu	t1,a2,19ca <memchr+0x88>
    199c:	6398                	ld	a4,0(a5)
    199e:	8f29                	xor	a4,a4,a0
    19a0:	011706b3          	add	a3,a4,a7
    19a4:	fff74713          	not	a4,a4
    19a8:	8f75                	and	a4,a4,a3
    19aa:	01077733          	and	a4,a4,a6
    19ae:	d37d                	beqz	a4,1994 <memchr+0x52>
    19b0:	853e                	mv	a0,a5
    19b2:	97b2                	add	a5,a5,a2
    19b4:	a021                	j	19bc <memchr+0x7a>
    for (; n && *s != c; s++, n--)
    19b6:	0505                	addi	a0,a0,1
    19b8:	00f50763          	beq	a0,a5,19c6 <memchr+0x84>
    19bc:	00054703          	lbu	a4,0(a0)
    19c0:	feb71be3          	bne	a4,a1,19b6 <memchr+0x74>
    19c4:	8082                	ret
    return n ? (void *)s : 0;
    19c6:	4501                	li	a0,0
}
    19c8:	8082                	ret
    return n ? (void *)s : 0;
    19ca:	4501                	li	a0,0
    for (; n && *s != c; s++, n--)
    19cc:	f275                	bnez	a2,19b0 <memchr+0x6e>
}
    19ce:	8082                	ret

00000000000019d0 <strnlen>:

size_t strnlen(const char *s, size_t n)
{
    19d0:	1101                	addi	sp,sp,-32
    19d2:	e822                	sd	s0,16(sp)
    const char *p = memchr(s, 0, n);
    19d4:	862e                	mv	a2,a1
{
    19d6:	842e                	mv	s0,a1
    const char *p = memchr(s, 0, n);
    19d8:	4581                	li	a1,0
{
    19da:	e426                	sd	s1,8(sp)
    19dc:	ec06                	sd	ra,24(sp)
    19de:	84aa                	mv	s1,a0
    const char *p = memchr(s, 0, n);
    19e0:	f63ff0ef          	jal	ra,1942 <memchr>
    return p ? p - s : n;
    19e4:	c519                	beqz	a0,19f2 <strnlen+0x22>
}
    19e6:	60e2                	ld	ra,24(sp)
    19e8:	6442                	ld	s0,16(sp)
    return p ? p - s : n;
    19ea:	8d05                	sub	a0,a0,s1
}
    19ec:	64a2                	ld	s1,8(sp)
    19ee:	6105                	addi	sp,sp,32
    19f0:	8082                	ret
    19f2:	60e2                	ld	ra,24(sp)
    return p ? p - s : n;
    19f4:	8522                	mv	a0,s0
}
    19f6:	6442                	ld	s0,16(sp)
    19f8:	64a2                	ld	s1,8(sp)
    19fa:	6105                	addi	sp,sp,32
    19fc:	8082                	ret

00000000000019fe <strcpy>:
char *strcpy(char *restrict d, const char *s)
{
    typedef size_t __attribute__((__may_alias__)) word;
    word *wd;
    const word *ws;
    if ((uintptr_t)s % SS == (uintptr_t)d % SS)
    19fe:	00b547b3          	xor	a5,a0,a1
    1a02:	8b9d                	andi	a5,a5,7
    1a04:	eb95                	bnez	a5,1a38 <strcpy+0x3a>
    {
        for (; (uintptr_t)s % SS; s++, d++)
    1a06:	0075f793          	andi	a5,a1,7
    1a0a:	e7b1                	bnez	a5,1a56 <strcpy+0x58>
            if (!(*d = *s))
                return d;
        wd = (void *)d;
        ws = (const void *)s;
        for (; !HASZERO(*ws); *wd++ = *ws++)
    1a0c:	6198                	ld	a4,0(a1)
    1a0e:	00000617          	auipc	a2,0x0
    1a12:	5da63603          	ld	a2,1498(a2) # 1fe8 <__clone+0xe8>
    1a16:	00000817          	auipc	a6,0x0
    1a1a:	5da83803          	ld	a6,1498(a6) # 1ff0 <__clone+0xf0>
    1a1e:	a029                	j	1a28 <strcpy+0x2a>
    1a20:	e118                	sd	a4,0(a0)
    1a22:	6598                	ld	a4,8(a1)
    1a24:	05a1                	addi	a1,a1,8
    1a26:	0521                	addi	a0,a0,8
    1a28:	00c707b3          	add	a5,a4,a2
    1a2c:	fff74693          	not	a3,a4
    1a30:	8ff5                	and	a5,a5,a3
    1a32:	0107f7b3          	and	a5,a5,a6
    1a36:	d7ed                	beqz	a5,1a20 <strcpy+0x22>
            ;
        d = (void *)wd;
        s = (const void *)ws;
    }
    for (; (*d = *s); s++, d++)
    1a38:	0005c783          	lbu	a5,0(a1)
    1a3c:	00f50023          	sb	a5,0(a0)
    1a40:	c785                	beqz	a5,1a68 <strcpy+0x6a>
    1a42:	0015c783          	lbu	a5,1(a1)
    1a46:	0505                	addi	a0,a0,1
    1a48:	0585                	addi	a1,a1,1
    1a4a:	00f50023          	sb	a5,0(a0)
    1a4e:	fbf5                	bnez	a5,1a42 <strcpy+0x44>
        ;
    return d;
}
    1a50:	8082                	ret
        for (; (uintptr_t)s % SS; s++, d++)
    1a52:	0505                	addi	a0,a0,1
    1a54:	df45                	beqz	a4,1a0c <strcpy+0xe>
            if (!(*d = *s))
    1a56:	0005c783          	lbu	a5,0(a1)
        for (; (uintptr_t)s % SS; s++, d++)
    1a5a:	0585                	addi	a1,a1,1
    1a5c:	0075f713          	andi	a4,a1,7
            if (!(*d = *s))
    1a60:	00f50023          	sb	a5,0(a0)
    1a64:	f7fd                	bnez	a5,1a52 <strcpy+0x54>
}
    1a66:	8082                	ret
    1a68:	8082                	ret

0000000000001a6a <strncpy>:
char *strncpy(char *restrict d, const char *s, size_t n)
{
    typedef size_t __attribute__((__may_alias__)) word;
    word *wd;
    const word *ws;
    if (((uintptr_t)s & ALIGN) == ((uintptr_t)d & ALIGN))
    1a6a:	00b547b3          	xor	a5,a0,a1
    1a6e:	8b9d                	andi	a5,a5,7
    1a70:	1a079863          	bnez	a5,1c20 <strncpy+0x1b6>
    {
        for (; ((uintptr_t)s & ALIGN) && n && (*d = *s); n--, s++, d++)
    1a74:	0075f793          	andi	a5,a1,7
    1a78:	16078463          	beqz	a5,1be0 <strncpy+0x176>
    1a7c:	ea01                	bnez	a2,1a8c <strncpy+0x22>
    1a7e:	a421                	j	1c86 <strncpy+0x21c>
    1a80:	167d                	addi	a2,a2,-1
    1a82:	0505                	addi	a0,a0,1
    1a84:	14070e63          	beqz	a4,1be0 <strncpy+0x176>
    1a88:	1a060863          	beqz	a2,1c38 <strncpy+0x1ce>
    1a8c:	0005c783          	lbu	a5,0(a1)
    1a90:	0585                	addi	a1,a1,1
    1a92:	0075f713          	andi	a4,a1,7
    1a96:	00f50023          	sb	a5,0(a0)
    1a9a:	f3fd                	bnez	a5,1a80 <strncpy+0x16>
    1a9c:	4805                	li	a6,1
    1a9e:	1a061863          	bnez	a2,1c4e <strncpy+0x1e4>
    1aa2:	40a007b3          	neg	a5,a0
    1aa6:	8b9d                	andi	a5,a5,7
    1aa8:	4681                	li	a3,0
    1aaa:	18061a63          	bnez	a2,1c3e <strncpy+0x1d4>
    1aae:	00778713          	addi	a4,a5,7
    1ab2:	45ad                	li	a1,11
    1ab4:	18b76363          	bltu	a4,a1,1c3a <strncpy+0x1d0>
    1ab8:	1ae6eb63          	bltu	a3,a4,1c6e <strncpy+0x204>
    1abc:	1a078363          	beqz	a5,1c62 <strncpy+0x1f8>
    for (int i = 0; i < n; ++i, *(p++) = c)
    1ac0:	00050023          	sb	zero,0(a0)
    1ac4:	4685                	li	a3,1
    1ac6:	00150713          	addi	a4,a0,1
    1aca:	18d78f63          	beq	a5,a3,1c68 <strncpy+0x1fe>
    1ace:	000500a3          	sb	zero,1(a0)
    1ad2:	4689                	li	a3,2
    1ad4:	00250713          	addi	a4,a0,2
    1ad8:	18d78e63          	beq	a5,a3,1c74 <strncpy+0x20a>
    1adc:	00050123          	sb	zero,2(a0)
    1ae0:	468d                	li	a3,3
    1ae2:	00350713          	addi	a4,a0,3
    1ae6:	16d78c63          	beq	a5,a3,1c5e <strncpy+0x1f4>
    1aea:	000501a3          	sb	zero,3(a0)
    1aee:	4691                	li	a3,4
    1af0:	00450713          	addi	a4,a0,4
    1af4:	18d78263          	beq	a5,a3,1c78 <strncpy+0x20e>
    1af8:	00050223          	sb	zero,4(a0)
    1afc:	4695                	li	a3,5
    1afe:	00550713          	addi	a4,a0,5
    1b02:	16d78d63          	beq	a5,a3,1c7c <strncpy+0x212>
    1b06:	000502a3          	sb	zero,5(a0)
    1b0a:	469d                	li	a3,7
    1b0c:	00650713          	addi	a4,a0,6
    1b10:	16d79863          	bne	a5,a3,1c80 <strncpy+0x216>
    1b14:	00750713          	addi	a4,a0,7
    1b18:	00050323          	sb	zero,6(a0)
    1b1c:	40f80833          	sub	a6,a6,a5
    1b20:	ff887593          	andi	a1,a6,-8
    1b24:	97aa                	add	a5,a5,a0
    1b26:	95be                	add	a1,a1,a5
    1b28:	0007b023          	sd	zero,0(a5)
    1b2c:	07a1                	addi	a5,a5,8
    1b2e:	feb79de3          	bne	a5,a1,1b28 <strncpy+0xbe>
    1b32:	ff887593          	andi	a1,a6,-8
    1b36:	9ead                	addw	a3,a3,a1
    1b38:	00b707b3          	add	a5,a4,a1
    1b3c:	12b80863          	beq	a6,a1,1c6c <strncpy+0x202>
    1b40:	00078023          	sb	zero,0(a5)
    1b44:	0016871b          	addiw	a4,a3,1
    1b48:	0ec77863          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b4c:	000780a3          	sb	zero,1(a5)
    1b50:	0026871b          	addiw	a4,a3,2
    1b54:	0ec77263          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b58:	00078123          	sb	zero,2(a5)
    1b5c:	0036871b          	addiw	a4,a3,3
    1b60:	0cc77c63          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b64:	000781a3          	sb	zero,3(a5)
    1b68:	0046871b          	addiw	a4,a3,4
    1b6c:	0cc77663          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b70:	00078223          	sb	zero,4(a5)
    1b74:	0056871b          	addiw	a4,a3,5
    1b78:	0cc77063          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b7c:	000782a3          	sb	zero,5(a5)
    1b80:	0066871b          	addiw	a4,a3,6
    1b84:	0ac77a63          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b88:	00078323          	sb	zero,6(a5)
    1b8c:	0076871b          	addiw	a4,a3,7
    1b90:	0ac77463          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1b94:	000783a3          	sb	zero,7(a5)
    1b98:	0086871b          	addiw	a4,a3,8
    1b9c:	08c77e63          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1ba0:	00078423          	sb	zero,8(a5)
    1ba4:	0096871b          	addiw	a4,a3,9
    1ba8:	08c77863          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1bac:	000784a3          	sb	zero,9(a5)
    1bb0:	00a6871b          	addiw	a4,a3,10
    1bb4:	08c77263          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1bb8:	00078523          	sb	zero,10(a5)
    1bbc:	00b6871b          	addiw	a4,a3,11
    1bc0:	06c77c63          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1bc4:	000785a3          	sb	zero,11(a5)
    1bc8:	00c6871b          	addiw	a4,a3,12
    1bcc:	06c77663          	bgeu	a4,a2,1c38 <strncpy+0x1ce>
    1bd0:	00078623          	sb	zero,12(a5)
    1bd4:	26b5                	addiw	a3,a3,13
    1bd6:	06c6f163          	bgeu	a3,a2,1c38 <strncpy+0x1ce>
    1bda:	000786a3          	sb	zero,13(a5)
    1bde:	8082                	ret
            ;
        if (!n || !*s)
    1be0:	c645                	beqz	a2,1c88 <strncpy+0x21e>
    1be2:	0005c783          	lbu	a5,0(a1)
    1be6:	ea078be3          	beqz	a5,1a9c <strncpy+0x32>
            goto tail;
        wd = (void *)d;
        ws = (const void *)s;
        for (; n >= sizeof(size_t) && !HASZERO(*ws); n -= sizeof(size_t), ws++, wd++)
    1bea:	479d                	li	a5,7
    1bec:	02c7ff63          	bgeu	a5,a2,1c2a <strncpy+0x1c0>
    1bf0:	00000897          	auipc	a7,0x0
    1bf4:	3f88b883          	ld	a7,1016(a7) # 1fe8 <__clone+0xe8>
    1bf8:	00000817          	auipc	a6,0x0
    1bfc:	3f883803          	ld	a6,1016(a6) # 1ff0 <__clone+0xf0>
    1c00:	431d                	li	t1,7
    1c02:	6198                	ld	a4,0(a1)
    1c04:	011707b3          	add	a5,a4,a7
    1c08:	fff74693          	not	a3,a4
    1c0c:	8ff5                	and	a5,a5,a3
    1c0e:	0107f7b3          	and	a5,a5,a6
    1c12:	ef81                	bnez	a5,1c2a <strncpy+0x1c0>
            *wd = *ws;
    1c14:	e118                	sd	a4,0(a0)
        for (; n >= sizeof(size_t) && !HASZERO(*ws); n -= sizeof(size_t), ws++, wd++)
    1c16:	1661                	addi	a2,a2,-8
    1c18:	05a1                	addi	a1,a1,8
    1c1a:	0521                	addi	a0,a0,8
    1c1c:	fec363e3          	bltu	t1,a2,1c02 <strncpy+0x198>
        d = (void *)wd;
        s = (const void *)ws;
    }
    for (; n && (*d = *s); n--, s++, d++)
    1c20:	e609                	bnez	a2,1c2a <strncpy+0x1c0>
    1c22:	a08d                	j	1c84 <strncpy+0x21a>
    1c24:	167d                	addi	a2,a2,-1
    1c26:	0505                	addi	a0,a0,1
    1c28:	ca01                	beqz	a2,1c38 <strncpy+0x1ce>
    1c2a:	0005c783          	lbu	a5,0(a1)
    1c2e:	0585                	addi	a1,a1,1
    1c30:	00f50023          	sb	a5,0(a0)
    1c34:	fbe5                	bnez	a5,1c24 <strncpy+0x1ba>
        ;
tail:
    1c36:	b59d                	j	1a9c <strncpy+0x32>
    memset(d, 0, n);
    return d;
}
    1c38:	8082                	ret
    1c3a:	472d                	li	a4,11
    1c3c:	bdb5                	j	1ab8 <strncpy+0x4e>
    1c3e:	00778713          	addi	a4,a5,7
    1c42:	45ad                	li	a1,11
    1c44:	fff60693          	addi	a3,a2,-1
    1c48:	e6b778e3          	bgeu	a4,a1,1ab8 <strncpy+0x4e>
    1c4c:	b7fd                	j	1c3a <strncpy+0x1d0>
    1c4e:	40a007b3          	neg	a5,a0
    1c52:	8832                	mv	a6,a2
    1c54:	8b9d                	andi	a5,a5,7
    1c56:	4681                	li	a3,0
    1c58:	e4060be3          	beqz	a2,1aae <strncpy+0x44>
    1c5c:	b7cd                	j	1c3e <strncpy+0x1d4>
    for (int i = 0; i < n; ++i, *(p++) = c)
    1c5e:	468d                	li	a3,3
    1c60:	bd75                	j	1b1c <strncpy+0xb2>
    1c62:	872a                	mv	a4,a0
    1c64:	4681                	li	a3,0
    1c66:	bd5d                	j	1b1c <strncpy+0xb2>
    1c68:	4685                	li	a3,1
    1c6a:	bd4d                	j	1b1c <strncpy+0xb2>
    1c6c:	8082                	ret
    1c6e:	87aa                	mv	a5,a0
    1c70:	4681                	li	a3,0
    1c72:	b5f9                	j	1b40 <strncpy+0xd6>
    1c74:	4689                	li	a3,2
    1c76:	b55d                	j	1b1c <strncpy+0xb2>
    1c78:	4691                	li	a3,4
    1c7a:	b54d                	j	1b1c <strncpy+0xb2>
    1c7c:	4695                	li	a3,5
    1c7e:	bd79                	j	1b1c <strncpy+0xb2>
    1c80:	4699                	li	a3,6
    1c82:	bd69                	j	1b1c <strncpy+0xb2>
    1c84:	8082                	ret
    1c86:	8082                	ret
    1c88:	8082                	ret

0000000000001c8a <open>:
#include <unistd.h>

#include "syscall.h"

int open(const char *path, int flags)
{
    1c8a:	87aa                	mv	a5,a0
    1c8c:	862e                	mv	a2,a1
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
    register long a7 __asm__("a7") = n;
    1c8e:	03800893          	li	a7,56
    register long a0 __asm__("a0") = a;
    1c92:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1c96:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    1c98:	4689                	li	a3,2
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1c9a:	00000073          	ecall
    return syscall(SYS_openat, AT_FDCWD, path, flags, O_RDWR);
}
    1c9e:	2501                	sext.w	a0,a0
    1ca0:	8082                	ret

0000000000001ca2 <openat>:
    register long a7 __asm__("a7") = n;
    1ca2:	03800893          	li	a7,56
    register long a3 __asm__("a3") = d;
    1ca6:	18000693          	li	a3,384
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1caa:	00000073          	ecall

int openat(int dirfd,const char *path, int flags)
{
    return syscall(SYS_openat, dirfd, path, flags, 0600);
}
    1cae:	2501                	sext.w	a0,a0
    1cb0:	8082                	ret

0000000000001cb2 <close>:
    register long a7 __asm__("a7") = n;
    1cb2:	03900893          	li	a7,57
    __asm_syscall("r"(a7), "0"(a0))
    1cb6:	00000073          	ecall

int close(int fd)
{
    return syscall(SYS_close, fd);
}
    1cba:	2501                	sext.w	a0,a0
    1cbc:	8082                	ret

0000000000001cbe <read>:
    register long a7 __asm__("a7") = n;
    1cbe:	03f00893          	li	a7,63
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1cc2:	00000073          	ecall

ssize_t read(int fd, void *buf, size_t len)
{
    return syscall(SYS_read, fd, buf, len);
}
    1cc6:	8082                	ret

0000000000001cc8 <write>:
    register long a7 __asm__("a7") = n;
    1cc8:	04000893          	li	a7,64
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1ccc:	00000073          	ecall

ssize_t write(int fd, const void *buf, size_t len)
{
    return syscall(SYS_write, fd, buf, len);
}
    1cd0:	8082                	ret

0000000000001cd2 <getpid>:
    register long a7 __asm__("a7") = n;
    1cd2:	0ac00893          	li	a7,172
    __asm_syscall("r"(a7))
    1cd6:	00000073          	ecall

pid_t getpid(void)
{
    return syscall(SYS_getpid);
}
    1cda:	2501                	sext.w	a0,a0
    1cdc:	8082                	ret

0000000000001cde <getppid>:
    register long a7 __asm__("a7") = n;
    1cde:	0ad00893          	li	a7,173
    __asm_syscall("r"(a7))
    1ce2:	00000073          	ecall

pid_t getppid(void)
{
    return syscall(SYS_getppid);
}
    1ce6:	2501                	sext.w	a0,a0
    1ce8:	8082                	ret

0000000000001cea <sched_yield>:
    register long a7 __asm__("a7") = n;
    1cea:	07c00893          	li	a7,124
    __asm_syscall("r"(a7))
    1cee:	00000073          	ecall

int sched_yield(void)
{
    return syscall(SYS_sched_yield);
}
    1cf2:	2501                	sext.w	a0,a0
    1cf4:	8082                	ret

0000000000001cf6 <fork>:
    register long a7 __asm__("a7") = n;
    1cf6:	0dc00893          	li	a7,220
    register long a0 __asm__("a0") = a;
    1cfa:	4545                	li	a0,17
    register long a1 __asm__("a1") = b;
    1cfc:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1cfe:	00000073          	ecall

pid_t fork(void)
{
    return syscall(SYS_clone, SIGCHLD, 0);
}
    1d02:	2501                	sext.w	a0,a0
    1d04:	8082                	ret

0000000000001d06 <clone>:

pid_t clone(int (*fn)(void *arg), void *arg, void *stack, size_t stack_size, unsigned long flags)
{
    1d06:	85b2                	mv	a1,a2
    1d08:	863a                	mv	a2,a4
    if (stack)
    1d0a:	c191                	beqz	a1,1d0e <clone+0x8>
	stack += stack_size;
    1d0c:	95b6                	add	a1,a1,a3

    return __clone(fn, stack, flags, NULL, NULL, NULL);
    1d0e:	4781                	li	a5,0
    1d10:	4701                	li	a4,0
    1d12:	4681                	li	a3,0
    1d14:	2601                	sext.w	a2,a2
    1d16:	a2ed                	j	1f00 <__clone>

0000000000001d18 <exit>:
    register long a7 __asm__("a7") = n;
    1d18:	05d00893          	li	a7,93
    __asm_syscall("r"(a7), "0"(a0))
    1d1c:	00000073          	ecall
    //return syscall(SYS_clone, fn, stack, flags, NULL, NULL, NULL);
}
void exit(int code)
{
    syscall(SYS_exit, code);
}
    1d20:	8082                	ret

0000000000001d22 <waitpid>:
    register long a7 __asm__("a7") = n;
    1d22:	10400893          	li	a7,260
    register long a3 __asm__("a3") = d;
    1d26:	4681                	li	a3,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1d28:	00000073          	ecall

int waitpid(int pid, int *code, int options)
{
    return syscall(SYS_wait4, pid, code, options, 0);
}
    1d2c:	2501                	sext.w	a0,a0
    1d2e:	8082                	ret

0000000000001d30 <exec>:
    register long a7 __asm__("a7") = n;
    1d30:	0dd00893          	li	a7,221
    __asm_syscall("r"(a7), "0"(a0))
    1d34:	00000073          	ecall

int exec(char *name)
{
    return syscall(SYS_execve, name);
}
    1d38:	2501                	sext.w	a0,a0
    1d3a:	8082                	ret

0000000000001d3c <execve>:
    register long a7 __asm__("a7") = n;
    1d3c:	0dd00893          	li	a7,221
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1d40:	00000073          	ecall

int execve(const char *name, char *const argv[], char *const argp[])
{
    return syscall(SYS_execve, name, argv, argp);
}
    1d44:	2501                	sext.w	a0,a0
    1d46:	8082                	ret

0000000000001d48 <times>:
    register long a7 __asm__("a7") = n;
    1d48:	09900893          	li	a7,153
    __asm_syscall("r"(a7), "0"(a0))
    1d4c:	00000073          	ecall

int times(void *mytimes)
{
	return syscall(SYS_times, mytimes);
}
    1d50:	2501                	sext.w	a0,a0
    1d52:	8082                	ret

0000000000001d54 <get_time>:

int64 get_time()
{
    1d54:	1141                	addi	sp,sp,-16
    register long a7 __asm__("a7") = n;
    1d56:	0a900893          	li	a7,169
    register long a0 __asm__("a0") = a;
    1d5a:	850a                	mv	a0,sp
    register long a1 __asm__("a1") = b;
    1d5c:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1d5e:	00000073          	ecall
    TimeVal time;
    int err = sys_get_time(&time, 0);
    if (err == 0)
    1d62:	2501                	sext.w	a0,a0
    1d64:	ed09                	bnez	a0,1d7e <get_time+0x2a>
    {
        return ((time.sec & 0xffff) * 1000 + time.usec / 1000);
    1d66:	67a2                	ld	a5,8(sp)
    1d68:	3e800713          	li	a4,1000
    1d6c:	00015503          	lhu	a0,0(sp)
    1d70:	02e7d7b3          	divu	a5,a5,a4
    1d74:	02e50533          	mul	a0,a0,a4
    1d78:	953e                	add	a0,a0,a5
    }
    else
    {
        return -1;
    }
}
    1d7a:	0141                	addi	sp,sp,16
    1d7c:	8082                	ret
        return -1;
    1d7e:	557d                	li	a0,-1
    1d80:	bfed                	j	1d7a <get_time+0x26>

0000000000001d82 <sys_get_time>:
    register long a7 __asm__("a7") = n;
    1d82:	0a900893          	li	a7,169
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1d86:	00000073          	ecall

int sys_get_time(TimeVal *ts, int tz)
{
    return syscall(SYS_gettimeofday, ts, tz);
}
    1d8a:	2501                	sext.w	a0,a0
    1d8c:	8082                	ret

0000000000001d8e <time>:
    register long a7 __asm__("a7") = n;
    1d8e:	42600893          	li	a7,1062
    __asm_syscall("r"(a7), "0"(a0))
    1d92:	00000073          	ecall

int time(unsigned long *tloc)
{
    return syscall(SYS_time, tloc);
}
    1d96:	2501                	sext.w	a0,a0
    1d98:	8082                	ret

0000000000001d9a <sleep>:

int sleep(unsigned long long time)
{
    1d9a:	1141                	addi	sp,sp,-16
    TimeVal tv = {.sec = time, .usec = 0};
    1d9c:	e02a                	sd	a0,0(sp)
    register long a0 __asm__("a0") = a;
    1d9e:	850a                	mv	a0,sp
    1da0:	e402                	sd	zero,8(sp)
    register long a7 __asm__("a7") = n;
    1da2:	06500893          	li	a7,101
    register long a1 __asm__("a1") = b;
    1da6:	85aa                	mv	a1,a0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1da8:	00000073          	ecall
    if (syscall(SYS_nanosleep, &tv, &tv)) return tv.sec;
    1dac:	e501                	bnez	a0,1db4 <sleep+0x1a>
    return 0;
    1dae:	4501                	li	a0,0
}
    1db0:	0141                	addi	sp,sp,16
    1db2:	8082                	ret
    if (syscall(SYS_nanosleep, &tv, &tv)) return tv.sec;
    1db4:	4502                	lw	a0,0(sp)
}
    1db6:	0141                	addi	sp,sp,16
    1db8:	8082                	ret

0000000000001dba <set_priority>:
    register long a7 __asm__("a7") = n;
    1dba:	08c00893          	li	a7,140
    __asm_syscall("r"(a7), "0"(a0))
    1dbe:	00000073          	ecall

int set_priority(int prio)
{
    return syscall(SYS_setpriority, prio);
}
    1dc2:	2501                	sext.w	a0,a0
    1dc4:	8082                	ret

0000000000001dc6 <mmap>:
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
    register long a7 __asm__("a7") = n;
    1dc6:	0de00893          	li	a7,222
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    register long a4 __asm__("a4") = e;
    register long a5 __asm__("a5") = f;
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5))
    1dca:	00000073          	ecall

void *mmap(void *start, size_t len, int prot, int flags, int fd, off_t off)
{
    return syscall(SYS_mmap, start, len, prot, flags, fd, off);
}
    1dce:	8082                	ret

0000000000001dd0 <munmap>:
    register long a7 __asm__("a7") = n;
    1dd0:	0d700893          	li	a7,215
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1dd4:	00000073          	ecall

int munmap(void *start, size_t len)
{
    return syscall(SYS_munmap, start, len);
}
    1dd8:	2501                	sext.w	a0,a0
    1dda:	8082                	ret

0000000000001ddc <wait>:

int wait(int *code)
{
    1ddc:	85aa                	mv	a1,a0
    register long a7 __asm__("a7") = n;
    1dde:	10400893          	li	a7,260
    register long a0 __asm__("a0") = a;
    1de2:	557d                	li	a0,-1
    register long a2 __asm__("a2") = c;
    1de4:	4601                	li	a2,0
    register long a3 __asm__("a3") = d;
    1de6:	4681                	li	a3,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1de8:	00000073          	ecall
    return waitpid((int)-1, code, 0);
}
    1dec:	2501                	sext.w	a0,a0
    1dee:	8082                	ret

0000000000001df0 <spawn>:
    register long a7 __asm__("a7") = n;
    1df0:	19000893          	li	a7,400
    __asm_syscall("r"(a7), "0"(a0))
    1df4:	00000073          	ecall

int spawn(char *file)
{
    return syscall(SYS_spawn, file);
}
    1df8:	2501                	sext.w	a0,a0
    1dfa:	8082                	ret

0000000000001dfc <mailread>:
    register long a7 __asm__("a7") = n;
    1dfc:	19100893          	li	a7,401
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e00:	00000073          	ecall

int mailread(void *buf, int len)
{
    return syscall(SYS_mailread, buf, len);
}
    1e04:	2501                	sext.w	a0,a0
    1e06:	8082                	ret

0000000000001e08 <mailwrite>:
    register long a7 __asm__("a7") = n;
    1e08:	19200893          	li	a7,402
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e0c:	00000073          	ecall

int mailwrite(int pid, void *buf, int len)
{
    return syscall(SYS_mailwrite, pid, buf, len);
}
    1e10:	2501                	sext.w	a0,a0
    1e12:	8082                	ret

0000000000001e14 <fstat>:
    register long a7 __asm__("a7") = n;
    1e14:	05000893          	li	a7,80
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e18:	00000073          	ecall

int fstat(int fd, struct kstat *st)
{
    return syscall(SYS_fstat, fd, st);
}
    1e1c:	2501                	sext.w	a0,a0
    1e1e:	8082                	ret

0000000000001e20 <sys_linkat>:
    register long a4 __asm__("a4") = e;
    1e20:	1702                	slli	a4,a4,0x20
    register long a7 __asm__("a7") = n;
    1e22:	02500893          	li	a7,37
    register long a4 __asm__("a4") = e;
    1e26:	9301                	srli	a4,a4,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1e28:	00000073          	ecall

int sys_linkat(int olddirfd, char *oldpath, int newdirfd, char *newpath, unsigned int flags)
{
    return syscall(SYS_linkat, olddirfd, oldpath, newdirfd, newpath, flags);
}
    1e2c:	2501                	sext.w	a0,a0
    1e2e:	8082                	ret

0000000000001e30 <sys_unlinkat>:
    register long a2 __asm__("a2") = c;
    1e30:	1602                	slli	a2,a2,0x20
    register long a7 __asm__("a7") = n;
    1e32:	02300893          	li	a7,35
    register long a2 __asm__("a2") = c;
    1e36:	9201                	srli	a2,a2,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e38:	00000073          	ecall

int sys_unlinkat(int dirfd, char *path, unsigned int flags)
{
    return syscall(SYS_unlinkat, dirfd, path, flags);
}
    1e3c:	2501                	sext.w	a0,a0
    1e3e:	8082                	ret

0000000000001e40 <link>:

int link(char *old_path, char *new_path)
{
    1e40:	87aa                	mv	a5,a0
    1e42:	86ae                	mv	a3,a1
    register long a7 __asm__("a7") = n;
    1e44:	02500893          	li	a7,37
    register long a0 __asm__("a0") = a;
    1e48:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1e4c:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    1e4e:	f9c00613          	li	a2,-100
    register long a4 __asm__("a4") = e;
    1e52:	4701                	li	a4,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1e54:	00000073          	ecall
    return sys_linkat(AT_FDCWD, old_path, AT_FDCWD, new_path, 0);
}
    1e58:	2501                	sext.w	a0,a0
    1e5a:	8082                	ret

0000000000001e5c <unlink>:

int unlink(char *path)
{
    1e5c:	85aa                	mv	a1,a0
    register long a7 __asm__("a7") = n;
    1e5e:	02300893          	li	a7,35
    register long a0 __asm__("a0") = a;
    1e62:	f9c00513          	li	a0,-100
    register long a2 __asm__("a2") = c;
    1e66:	4601                	li	a2,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e68:	00000073          	ecall
    return sys_unlinkat(AT_FDCWD, path, 0);
}
    1e6c:	2501                	sext.w	a0,a0
    1e6e:	8082                	ret

0000000000001e70 <uname>:
    register long a7 __asm__("a7") = n;
    1e70:	0a000893          	li	a7,160
    __asm_syscall("r"(a7), "0"(a0))
    1e74:	00000073          	ecall

int uname(void *buf)
{
    return syscall(SYS_uname, buf);
}
    1e78:	2501                	sext.w	a0,a0
    1e7a:	8082                	ret

0000000000001e7c <brk>:
    register long a7 __asm__("a7") = n;
    1e7c:	0d600893          	li	a7,214
    __asm_syscall("r"(a7), "0"(a0))
    1e80:	00000073          	ecall

int brk(void *addr)
{
    return syscall(SYS_brk, addr);
}
    1e84:	2501                	sext.w	a0,a0
    1e86:	8082                	ret

0000000000001e88 <getcwd>:
    register long a7 __asm__("a7") = n;
    1e88:	48c5                	li	a7,17
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e8a:	00000073          	ecall

char *getcwd(char *buf, size_t size){
    return syscall(SYS_getcwd, buf, size);
}
    1e8e:	8082                	ret

0000000000001e90 <chdir>:
    register long a7 __asm__("a7") = n;
    1e90:	03100893          	li	a7,49
    __asm_syscall("r"(a7), "0"(a0))
    1e94:	00000073          	ecall

int chdir(const char *path){
    return syscall(SYS_chdir, path);
}
    1e98:	2501                	sext.w	a0,a0
    1e9a:	8082                	ret

0000000000001e9c <mkdir>:

int mkdir(const char *path, mode_t mode){
    1e9c:	862e                	mv	a2,a1
    1e9e:	87aa                	mv	a5,a0
    register long a2 __asm__("a2") = c;
    1ea0:	1602                	slli	a2,a2,0x20
    register long a7 __asm__("a7") = n;
    1ea2:	02200893          	li	a7,34
    register long a0 __asm__("a0") = a;
    1ea6:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1eaa:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    1eac:	9201                	srli	a2,a2,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1eae:	00000073          	ecall
    return syscall(SYS_mkdirat, AT_FDCWD, path, mode);
}
    1eb2:	2501                	sext.w	a0,a0
    1eb4:	8082                	ret

0000000000001eb6 <getdents>:
    register long a7 __asm__("a7") = n;
    1eb6:	03d00893          	li	a7,61
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1eba:	00000073          	ecall

int getdents(int fd, struct linux_dirent64 *dirp64, unsigned long len){
    //return syscall(SYS_getdents64, fd, dirp64, len);
    return syscall(SYS_getdents64, fd, dirp64, len);
}
    1ebe:	2501                	sext.w	a0,a0
    1ec0:	8082                	ret

0000000000001ec2 <pipe>:
    register long a7 __asm__("a7") = n;
    1ec2:	03b00893          	li	a7,59
    register long a1 __asm__("a1") = b;
    1ec6:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1ec8:	00000073          	ecall

int pipe(int fd[2]){
    return syscall(SYS_pipe2, fd, 0);
}
    1ecc:	2501                	sext.w	a0,a0
    1ece:	8082                	ret

0000000000001ed0 <dup>:
    register long a7 __asm__("a7") = n;
    1ed0:	48dd                	li	a7,23
    __asm_syscall("r"(a7), "0"(a0))
    1ed2:	00000073          	ecall

int dup(int fd){
    return syscall(SYS_dup, fd);
}
    1ed6:	2501                	sext.w	a0,a0
    1ed8:	8082                	ret

0000000000001eda <dup2>:
    register long a7 __asm__("a7") = n;
    1eda:	48e1                	li	a7,24
    register long a2 __asm__("a2") = c;
    1edc:	4601                	li	a2,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1ede:	00000073          	ecall

int dup2(int old, int new){
    return syscall(SYS_dup3, old, new, 0);
}
    1ee2:	2501                	sext.w	a0,a0
    1ee4:	8082                	ret

0000000000001ee6 <mount>:
    register long a7 __asm__("a7") = n;
    1ee6:	02800893          	li	a7,40
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1eea:	00000073          	ecall

int mount(const char *special, const char *dir, const char *fstype, unsigned long flags, const void *data)
{
        return syscall(SYS_mount, special, dir, fstype, flags, data);
}
    1eee:	2501                	sext.w	a0,a0
    1ef0:	8082                	ret

0000000000001ef2 <umount>:
    register long a7 __asm__("a7") = n;
    1ef2:	02700893          	li	a7,39
    register long a1 __asm__("a1") = b;
    1ef6:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1ef8:	00000073          	ecall

int umount(const char *special)
{
        return syscall(SYS_umount2, special, 0);
}
    1efc:	2501                	sext.w	a0,a0
    1efe:	8082                	ret

0000000000001f00 <__clone>:

.global __clone
.type  __clone, %function
__clone:
	# Save func and arg to stack
	addi a1, a1, -16
    1f00:	15c1                	addi	a1,a1,-16
	sd a0, 0(a1)
    1f02:	e188                	sd	a0,0(a1)
	sd a3, 8(a1)
    1f04:	e594                	sd	a3,8(a1)

	# Call SYS_clone
	mv a0, a2
    1f06:	8532                	mv	a0,a2
	mv a2, a4
    1f08:	863a                	mv	a2,a4
	mv a3, a5
    1f0a:	86be                	mv	a3,a5
	mv a4, a6
    1f0c:	8742                	mv	a4,a6
	li a7, 220 # SYS_clone
    1f0e:	0dc00893          	li	a7,220
	ecall
    1f12:	00000073          	ecall

	beqz a0, 1f
    1f16:	c111                	beqz	a0,1f1a <__clone+0x1a>
	# Parent
	ret
    1f18:	8082                	ret

	# Child
1:      ld a1, 0(sp)
    1f1a:	6582                	ld	a1,0(sp)
	ld a0, 8(sp)
    1f1c:	6522                	ld	a0,8(sp)
	jalr a1
    1f1e:	9582                	jalr	a1

	# Exit
	li a7, 93 # SYS_exit
    1f20:	05d00893          	li	a7,93
	ecall
    1f24:	00000073          	ecall
