
/home/weihuan/Documents/testsuits-for-oskernel-preliminary/riscv-syscalls-testing/user/build/riscv64/pipe:     file format elf64-littleriscv


Disassembly of section .text:

0000000000001000 <_start>:
.section .text.entry
.globl _start
_start:
    mv a0, sp
    1000:	850a                	mv	a0,sp
    tail __start_main
    1002:	a281                	j	1142 <__start_main>

0000000000001004 <test_pipe>:
 * 成功测试时的输出：
 * "  Write to pipe successfully."
 */
static int fd[2];

void test_pipe(void){
    1004:	7135                	addi	sp,sp,-160
    TEST_START(__func__);
    1006:	00001517          	auipc	a0,0x1
    100a:	f7a50513          	addi	a0,a0,-134 # 1f80 <__clone+0x2c>
void test_pipe(void){
    100e:	ed06                	sd	ra,152(sp)
    1010:	e922                	sd	s0,144(sp)
    1012:	e526                	sd	s1,136(sp)
    TEST_START(__func__);
    1014:	3a0000ef          	jal	ra,13b4 <puts>
    1018:	00001517          	auipc	a0,0x1
    101c:	01850513          	addi	a0,a0,24 # 2030 <__func__.0>
    1020:	394000ef          	jal	ra,13b4 <puts>
    1024:	00001517          	auipc	a0,0x1
    1028:	f7450513          	addi	a0,a0,-140 # 1f98 <__clone+0x44>
    102c:	388000ef          	jal	ra,13b4 <puts>
    int cpid;
    char buf[128] = {0};
    int ret = pipe(fd);
    1030:	00001517          	auipc	a0,0x1
    1034:	ff850513          	addi	a0,a0,-8 # 2028 <fd>
    char buf[128] = {0};
    1038:	e002                	sd	zero,0(sp)
    103a:	e402                	sd	zero,8(sp)
    103c:	e802                	sd	zero,16(sp)
    103e:	ec02                	sd	zero,24(sp)
    1040:	f002                	sd	zero,32(sp)
    1042:	f402                	sd	zero,40(sp)
    1044:	f802                	sd	zero,48(sp)
    1046:	fc02                	sd	zero,56(sp)
    1048:	e082                	sd	zero,64(sp)
    104a:	e482                	sd	zero,72(sp)
    104c:	e882                	sd	zero,80(sp)
    104e:	ec82                	sd	zero,88(sp)
    1050:	f082                	sd	zero,96(sp)
    1052:	f482                	sd	zero,104(sp)
    1054:	f882                	sd	zero,112(sp)
    1056:	fc82                	sd	zero,120(sp)
    int ret = pipe(fd);
    1058:	6bf000ef          	jal	ra,1f16 <pipe>
    assert(ret != -1);
    105c:	57fd                	li	a5,-1
    105e:	0cf50363          	beq	a0,a5,1124 <test_pipe+0x120>
    const char *data = "  Write to pipe successfully.\n";
    cpid = fork();
    1062:	4e9000ef          	jal	ra,1d4a <fork>
    1066:	842a                	mv	s0,a0
    printf("cpid: %d\n", cpid);
    1068:	85aa                	mv	a1,a0
    106a:	00001517          	auipc	a0,0x1
    106e:	f5e50513          	addi	a0,a0,-162 # 1fc8 <__clone+0x74>
    1072:	364000ef          	jal	ra,13d6 <printf>
    if(cpid > 0){
    1076:	06805a63          	blez	s0,10ea <test_pipe+0xe6>
	close(fd[1]);
    107a:	00001417          	auipc	s0,0x1
    107e:	fae40413          	addi	s0,s0,-82 # 2028 <fd>
    1082:	4048                	lw	a0,4(s0)
    1084:	483000ef          	jal	ra,1d06 <close>
	while(read(fd[0], buf, 1) > 0)
    1088:	a019                	j	108e <test_pipe+0x8a>
            write(STDOUT, buf, 1);
    108a:	493000ef          	jal	ra,1d1c <write>
	while(read(fd[0], buf, 1) > 0)
    108e:	4008                	lw	a0,0(s0)
    1090:	4605                	li	a2,1
    1092:	858a                	mv	a1,sp
    1094:	47f000ef          	jal	ra,1d12 <read>
    1098:	87aa                	mv	a5,a0
            write(STDOUT, buf, 1);
    109a:	4605                	li	a2,1
    109c:	858a                	mv	a1,sp
    109e:	4505                	li	a0,1
	while(read(fd[0], buf, 1) > 0)
    10a0:	fef045e3          	bgtz	a5,108a <test_pipe+0x86>
	write(STDOUT, "\n", 1);
    10a4:	00001597          	auipc	a1,0x1
    10a8:	f1c58593          	addi	a1,a1,-228 # 1fc0 <__clone+0x6c>
    10ac:	471000ef          	jal	ra,1d1c <write>
	close(fd[0]);
    10b0:	4008                	lw	a0,0(s0)
    10b2:	455000ef          	jal	ra,1d06 <close>
	wait(NULL);
    10b6:	4501                	li	a0,0
    10b8:	579000ef          	jal	ra,1e30 <wait>
	close(fd[0]);
	write(fd[1], data, strlen(data));
	close(fd[1]);
	exit(0);
    }
    TEST_END(__func__);
    10bc:	00001517          	auipc	a0,0x1
    10c0:	f3c50513          	addi	a0,a0,-196 # 1ff8 <__clone+0xa4>
    10c4:	2f0000ef          	jal	ra,13b4 <puts>
    10c8:	00001517          	auipc	a0,0x1
    10cc:	f6850513          	addi	a0,a0,-152 # 2030 <__func__.0>
    10d0:	2e4000ef          	jal	ra,13b4 <puts>
    10d4:	00001517          	auipc	a0,0x1
    10d8:	ec450513          	addi	a0,a0,-316 # 1f98 <__clone+0x44>
    10dc:	2d8000ef          	jal	ra,13b4 <puts>
}
    10e0:	60ea                	ld	ra,152(sp)
    10e2:	644a                	ld	s0,144(sp)
    10e4:	64aa                	ld	s1,136(sp)
    10e6:	610d                	addi	sp,sp,160
    10e8:	8082                	ret
	close(fd[0]);
    10ea:	00001417          	auipc	s0,0x1
    10ee:	f3e40413          	addi	s0,s0,-194 # 2028 <fd>
    10f2:	4008                	lw	a0,0(s0)
    10f4:	413000ef          	jal	ra,1d06 <close>
	write(fd[1], data, strlen(data));
    10f8:	4044                	lw	s1,4(s0)
    10fa:	00001517          	auipc	a0,0x1
    10fe:	ede50513          	addi	a0,a0,-290 # 1fd8 <__clone+0x84>
    1102:	037000ef          	jal	ra,1938 <strlen>
    1106:	862a                	mv	a2,a0
    1108:	00001597          	auipc	a1,0x1
    110c:	ed058593          	addi	a1,a1,-304 # 1fd8 <__clone+0x84>
    1110:	8526                	mv	a0,s1
    1112:	40b000ef          	jal	ra,1d1c <write>
	close(fd[1]);
    1116:	4048                	lw	a0,4(s0)
    1118:	3ef000ef          	jal	ra,1d06 <close>
	exit(0);
    111c:	4501                	li	a0,0
    111e:	44f000ef          	jal	ra,1d6c <exit>
    1122:	bf69                	j	10bc <test_pipe+0xb8>
    assert(ret != -1);
    1124:	00001517          	auipc	a0,0x1
    1128:	e8450513          	addi	a0,a0,-380 # 1fa8 <__clone+0x54>
    112c:	52e000ef          	jal	ra,165a <panic>
    1130:	bf0d                	j	1062 <test_pipe+0x5e>

0000000000001132 <main>:

int main(void){
    1132:	1141                	addi	sp,sp,-16
    1134:	e406                	sd	ra,8(sp)
    test_pipe();
    1136:	ecfff0ef          	jal	ra,1004 <test_pipe>
    return 0;
}
    113a:	60a2                	ld	ra,8(sp)
    113c:	4501                	li	a0,0
    113e:	0141                	addi	sp,sp,16
    1140:	8082                	ret

0000000000001142 <__start_main>:
#include <unistd.h>

extern int main();

int __start_main(long *p)
{
    1142:	85aa                	mv	a1,a0
	int argc = p[0];
	char **argv = (void *)(p+1);

	exit(main(argc, argv));
    1144:	4108                	lw	a0,0(a0)
{
    1146:	1141                	addi	sp,sp,-16
	exit(main(argc, argv));
    1148:	05a1                	addi	a1,a1,8
{
    114a:	e406                	sd	ra,8(sp)
	exit(main(argc, argv));
    114c:	fe7ff0ef          	jal	ra,1132 <main>
    1150:	41d000ef          	jal	ra,1d6c <exit>
	return 0;
}
    1154:	60a2                	ld	ra,8(sp)
    1156:	4501                	li	a0,0
    1158:	0141                	addi	sp,sp,16
    115a:	8082                	ret

000000000000115c <printint.constprop.0>:
    write(f, s, l);
}

static char digits[] = "0123456789abcdef";

static void printint(int xx, int base, int sign)
    115c:	7179                	addi	sp,sp,-48
    115e:	f406                	sd	ra,40(sp)
{
    char buf[16 + 1];
    int i;
    uint x;

    if (sign && (sign = xx < 0))
    1160:	12054b63          	bltz	a0,1296 <printint.constprop.0+0x13a>

    buf[16] = 0;
    i = 15;
    do
    {
        buf[i--] = digits[x % base];
    1164:	02b577bb          	remuw	a5,a0,a1
    1168:	00001617          	auipc	a2,0x1
    116c:	ed860613          	addi	a2,a2,-296 # 2040 <digits>
    buf[16] = 0;
    1170:	00010c23          	sb	zero,24(sp)
        buf[i--] = digits[x % base];
    1174:	0005871b          	sext.w	a4,a1
    1178:	1782                	slli	a5,a5,0x20
    117a:	9381                	srli	a5,a5,0x20
    117c:	97b2                	add	a5,a5,a2
    117e:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    1182:	02b5583b          	divuw	a6,a0,a1
        buf[i--] = digits[x % base];
    1186:	00f10ba3          	sb	a5,23(sp)
    } while ((x /= base) != 0);
    118a:	1cb56363          	bltu	a0,a1,1350 <printint.constprop.0+0x1f4>
        buf[i--] = digits[x % base];
    118e:	45b9                	li	a1,14
    1190:	02e877bb          	remuw	a5,a6,a4
    1194:	1782                	slli	a5,a5,0x20
    1196:	9381                	srli	a5,a5,0x20
    1198:	97b2                	add	a5,a5,a2
    119a:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    119e:	02e856bb          	divuw	a3,a6,a4
        buf[i--] = digits[x % base];
    11a2:	00f10b23          	sb	a5,22(sp)
    } while ((x /= base) != 0);
    11a6:	0ce86e63          	bltu	a6,a4,1282 <printint.constprop.0+0x126>
        buf[i--] = digits[x % base];
    11aa:	02e6f5bb          	remuw	a1,a3,a4
    } while ((x /= base) != 0);
    11ae:	02e6d7bb          	divuw	a5,a3,a4
        buf[i--] = digits[x % base];
    11b2:	1582                	slli	a1,a1,0x20
    11b4:	9181                	srli	a1,a1,0x20
    11b6:	95b2                	add	a1,a1,a2
    11b8:	0005c583          	lbu	a1,0(a1)
    11bc:	00b10aa3          	sb	a1,21(sp)
    } while ((x /= base) != 0);
    11c0:	0007859b          	sext.w	a1,a5
    11c4:	12e6ec63          	bltu	a3,a4,12fc <printint.constprop.0+0x1a0>
        buf[i--] = digits[x % base];
    11c8:	02e7f6bb          	remuw	a3,a5,a4
    11cc:	1682                	slli	a3,a3,0x20
    11ce:	9281                	srli	a3,a3,0x20
    11d0:	96b2                	add	a3,a3,a2
    11d2:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11d6:	02e7d83b          	divuw	a6,a5,a4
        buf[i--] = digits[x % base];
    11da:	00d10a23          	sb	a3,20(sp)
    } while ((x /= base) != 0);
    11de:	12e5e863          	bltu	a1,a4,130e <printint.constprop.0+0x1b2>
        buf[i--] = digits[x % base];
    11e2:	02e876bb          	remuw	a3,a6,a4
    11e6:	1682                	slli	a3,a3,0x20
    11e8:	9281                	srli	a3,a3,0x20
    11ea:	96b2                	add	a3,a3,a2
    11ec:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    11f0:	02e855bb          	divuw	a1,a6,a4
        buf[i--] = digits[x % base];
    11f4:	00d109a3          	sb	a3,19(sp)
    } while ((x /= base) != 0);
    11f8:	12e86463          	bltu	a6,a4,1320 <printint.constprop.0+0x1c4>
        buf[i--] = digits[x % base];
    11fc:	02e5f6bb          	remuw	a3,a1,a4
    1200:	1682                	slli	a3,a3,0x20
    1202:	9281                	srli	a3,a3,0x20
    1204:	96b2                	add	a3,a3,a2
    1206:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    120a:	02e5d83b          	divuw	a6,a1,a4
        buf[i--] = digits[x % base];
    120e:	00d10923          	sb	a3,18(sp)
    } while ((x /= base) != 0);
    1212:	0ce5ec63          	bltu	a1,a4,12ea <printint.constprop.0+0x18e>
        buf[i--] = digits[x % base];
    1216:	02e876bb          	remuw	a3,a6,a4
    121a:	1682                	slli	a3,a3,0x20
    121c:	9281                	srli	a3,a3,0x20
    121e:	96b2                	add	a3,a3,a2
    1220:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    1224:	02e855bb          	divuw	a1,a6,a4
        buf[i--] = digits[x % base];
    1228:	00d108a3          	sb	a3,17(sp)
    } while ((x /= base) != 0);
    122c:	10e86963          	bltu	a6,a4,133e <printint.constprop.0+0x1e2>
        buf[i--] = digits[x % base];
    1230:	02e5f6bb          	remuw	a3,a1,a4
    1234:	1682                	slli	a3,a3,0x20
    1236:	9281                	srli	a3,a3,0x20
    1238:	96b2                	add	a3,a3,a2
    123a:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    123e:	02e5d83b          	divuw	a6,a1,a4
        buf[i--] = digits[x % base];
    1242:	00d10823          	sb	a3,16(sp)
    } while ((x /= base) != 0);
    1246:	10e5e763          	bltu	a1,a4,1354 <printint.constprop.0+0x1f8>
        buf[i--] = digits[x % base];
    124a:	02e876bb          	remuw	a3,a6,a4
    124e:	1682                	slli	a3,a3,0x20
    1250:	9281                	srli	a3,a3,0x20
    1252:	96b2                	add	a3,a3,a2
    1254:	0006c683          	lbu	a3,0(a3)
    } while ((x /= base) != 0);
    1258:	02e857bb          	divuw	a5,a6,a4
        buf[i--] = digits[x % base];
    125c:	00d107a3          	sb	a3,15(sp)
    } while ((x /= base) != 0);
    1260:	10e86363          	bltu	a6,a4,1366 <printint.constprop.0+0x20a>
        buf[i--] = digits[x % base];
    1264:	1782                	slli	a5,a5,0x20
    1266:	9381                	srli	a5,a5,0x20
    1268:	97b2                	add	a5,a5,a2
    126a:	0007c783          	lbu	a5,0(a5)
    126e:	4599                	li	a1,6
    1270:	00f10723          	sb	a5,14(sp)

    if (sign)
    1274:	00055763          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    1278:	02d00793          	li	a5,45
    127c:	00f106a3          	sb	a5,13(sp)
        buf[i--] = digits[x % base];
    1280:	4595                	li	a1,5
    write(f, s, l);
    1282:	003c                	addi	a5,sp,8
    1284:	4641                	li	a2,16
    1286:	9e0d                	subw	a2,a2,a1
    1288:	4505                	li	a0,1
    128a:	95be                	add	a1,a1,a5
    128c:	291000ef          	jal	ra,1d1c <write>
    i++;
    if (i < 0)
        puts("printint error");
    out(stdout, buf + i, 16 - i);
}
    1290:	70a2                	ld	ra,40(sp)
    1292:	6145                	addi	sp,sp,48
    1294:	8082                	ret
        x = -xx;
    1296:	40a0083b          	negw	a6,a0
        buf[i--] = digits[x % base];
    129a:	02b877bb          	remuw	a5,a6,a1
    129e:	00001617          	auipc	a2,0x1
    12a2:	da260613          	addi	a2,a2,-606 # 2040 <digits>
    buf[16] = 0;
    12a6:	00010c23          	sb	zero,24(sp)
        buf[i--] = digits[x % base];
    12aa:	0005871b          	sext.w	a4,a1
    12ae:	1782                	slli	a5,a5,0x20
    12b0:	9381                	srli	a5,a5,0x20
    12b2:	97b2                	add	a5,a5,a2
    12b4:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    12b8:	02b858bb          	divuw	a7,a6,a1
        buf[i--] = digits[x % base];
    12bc:	00f10ba3          	sb	a5,23(sp)
    } while ((x /= base) != 0);
    12c0:	06b86963          	bltu	a6,a1,1332 <printint.constprop.0+0x1d6>
        buf[i--] = digits[x % base];
    12c4:	02e8f7bb          	remuw	a5,a7,a4
    12c8:	1782                	slli	a5,a5,0x20
    12ca:	9381                	srli	a5,a5,0x20
    12cc:	97b2                	add	a5,a5,a2
    12ce:	0007c783          	lbu	a5,0(a5)
    } while ((x /= base) != 0);
    12d2:	02e8d6bb          	divuw	a3,a7,a4
        buf[i--] = digits[x % base];
    12d6:	00f10b23          	sb	a5,22(sp)
    } while ((x /= base) != 0);
    12da:	ece8f8e3          	bgeu	a7,a4,11aa <printint.constprop.0+0x4e>
        buf[i--] = '-';
    12de:	02d00793          	li	a5,45
    12e2:	00f10aa3          	sb	a5,21(sp)
        buf[i--] = digits[x % base];
    12e6:	45b5                	li	a1,13
    12e8:	bf69                	j	1282 <printint.constprop.0+0x126>
    12ea:	45a9                	li	a1,10
    if (sign)
    12ec:	f8055be3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    12f0:	02d00793          	li	a5,45
    12f4:	00f108a3          	sb	a5,17(sp)
        buf[i--] = digits[x % base];
    12f8:	45a5                	li	a1,9
    12fa:	b761                	j	1282 <printint.constprop.0+0x126>
    12fc:	45b5                	li	a1,13
    if (sign)
    12fe:	f80552e3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    1302:	02d00793          	li	a5,45
    1306:	00f10a23          	sb	a5,20(sp)
        buf[i--] = digits[x % base];
    130a:	45b1                	li	a1,12
    130c:	bf9d                	j	1282 <printint.constprop.0+0x126>
    130e:	45b1                	li	a1,12
    if (sign)
    1310:	f60559e3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    1314:	02d00793          	li	a5,45
    1318:	00f109a3          	sb	a5,19(sp)
        buf[i--] = digits[x % base];
    131c:	45ad                	li	a1,11
    131e:	b795                	j	1282 <printint.constprop.0+0x126>
    1320:	45ad                	li	a1,11
    if (sign)
    1322:	f60550e3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    1326:	02d00793          	li	a5,45
    132a:	00f10923          	sb	a5,18(sp)
        buf[i--] = digits[x % base];
    132e:	45a9                	li	a1,10
    1330:	bf89                	j	1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    1332:	02d00793          	li	a5,45
    1336:	00f10b23          	sb	a5,22(sp)
        buf[i--] = digits[x % base];
    133a:	45b9                	li	a1,14
    133c:	b799                	j	1282 <printint.constprop.0+0x126>
    133e:	45a5                	li	a1,9
    if (sign)
    1340:	f40551e3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    1344:	02d00793          	li	a5,45
    1348:	00f10823          	sb	a5,16(sp)
        buf[i--] = digits[x % base];
    134c:	45a1                	li	a1,8
    134e:	bf15                	j	1282 <printint.constprop.0+0x126>
    i = 15;
    1350:	45bd                	li	a1,15
    1352:	bf05                	j	1282 <printint.constprop.0+0x126>
        buf[i--] = digits[x % base];
    1354:	45a1                	li	a1,8
    if (sign)
    1356:	f20556e3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    135a:	02d00793          	li	a5,45
    135e:	00f107a3          	sb	a5,15(sp)
        buf[i--] = digits[x % base];
    1362:	459d                	li	a1,7
    1364:	bf39                	j	1282 <printint.constprop.0+0x126>
    1366:	459d                	li	a1,7
    if (sign)
    1368:	f0055de3          	bgez	a0,1282 <printint.constprop.0+0x126>
        buf[i--] = '-';
    136c:	02d00793          	li	a5,45
    1370:	00f10723          	sb	a5,14(sp)
        buf[i--] = digits[x % base];
    1374:	4599                	li	a1,6
    1376:	b731                	j	1282 <printint.constprop.0+0x126>

0000000000001378 <getchar>:
{
    1378:	1101                	addi	sp,sp,-32
    read(stdin, &byte, 1);
    137a:	00f10593          	addi	a1,sp,15
    137e:	4605                	li	a2,1
    1380:	4501                	li	a0,0
{
    1382:	ec06                	sd	ra,24(sp)
    char byte = 0;
    1384:	000107a3          	sb	zero,15(sp)
    read(stdin, &byte, 1);
    1388:	18b000ef          	jal	ra,1d12 <read>
}
    138c:	60e2                	ld	ra,24(sp)
    138e:	00f14503          	lbu	a0,15(sp)
    1392:	6105                	addi	sp,sp,32
    1394:	8082                	ret

0000000000001396 <putchar>:
{
    1396:	1101                	addi	sp,sp,-32
    1398:	87aa                	mv	a5,a0
    return write(stdout, &byte, 1);
    139a:	00f10593          	addi	a1,sp,15
    139e:	4605                	li	a2,1
    13a0:	4505                	li	a0,1
{
    13a2:	ec06                	sd	ra,24(sp)
    char byte = c;
    13a4:	00f107a3          	sb	a5,15(sp)
    return write(stdout, &byte, 1);
    13a8:	175000ef          	jal	ra,1d1c <write>
}
    13ac:	60e2                	ld	ra,24(sp)
    13ae:	2501                	sext.w	a0,a0
    13b0:	6105                	addi	sp,sp,32
    13b2:	8082                	ret

00000000000013b4 <puts>:
{
    13b4:	1141                	addi	sp,sp,-16
    13b6:	e406                	sd	ra,8(sp)
    13b8:	e022                	sd	s0,0(sp)
    13ba:	842a                	mv	s0,a0
    r = -(write(stdout, s, strlen(s)) < 0);
    13bc:	57c000ef          	jal	ra,1938 <strlen>
    13c0:	862a                	mv	a2,a0
    13c2:	85a2                	mv	a1,s0
    13c4:	4505                	li	a0,1
    13c6:	157000ef          	jal	ra,1d1c <write>
}
    13ca:	60a2                	ld	ra,8(sp)
    13cc:	6402                	ld	s0,0(sp)
    r = -(write(stdout, s, strlen(s)) < 0);
    13ce:	957d                	srai	a0,a0,0x3f
    return r;
    13d0:	2501                	sext.w	a0,a0
}
    13d2:	0141                	addi	sp,sp,16
    13d4:	8082                	ret

00000000000013d6 <printf>:
    out(stdout, buf, i);
}

// Print to the console. only understands %d, %x, %p, %s.
void printf(const char *fmt, ...)
{
    13d6:	7171                	addi	sp,sp,-176
    13d8:	fc56                	sd	s5,56(sp)
    13da:	ed3e                	sd	a5,152(sp)
    buf[i++] = '0';
    13dc:	7ae1                	lui	s5,0xffff8
    va_list ap;
    int cnt = 0, l = 0;
    char *a, *z, *s = (char *)fmt, str;
    int f = stdout;

    va_start(ap, fmt);
    13de:	18bc                	addi	a5,sp,120
{
    13e0:	e8ca                	sd	s2,80(sp)
    13e2:	e4ce                	sd	s3,72(sp)
    13e4:	e0d2                	sd	s4,64(sp)
    13e6:	f85a                	sd	s6,48(sp)
    13e8:	f486                	sd	ra,104(sp)
    13ea:	f0a2                	sd	s0,96(sp)
    13ec:	eca6                	sd	s1,88(sp)
    13ee:	fcae                	sd	a1,120(sp)
    13f0:	e132                	sd	a2,128(sp)
    13f2:	e536                	sd	a3,136(sp)
    13f4:	e93a                	sd	a4,144(sp)
    13f6:	f142                	sd	a6,160(sp)
    13f8:	f546                	sd	a7,168(sp)
    va_start(ap, fmt);
    13fa:	e03e                	sd	a5,0(sp)
    for (;;)
    {
        if (!*s)
            break;
        for (a = s; *s && *s != '%'; s++)
    13fc:	02500913          	li	s2,37
        out(f, a, l);
        if (l)
            continue;
        if (s[1] == 0)
            break;
        switch (s[1])
    1400:	07300a13          	li	s4,115
        case 'p':
            printptr(va_arg(ap, uint64));
            break;
        case 's':
            if ((a = va_arg(ap, char *)) == 0)
                a = "(null)";
    1404:	00001b17          	auipc	s6,0x1
    1408:	c04b0b13          	addi	s6,s6,-1020 # 2008 <__clone+0xb4>
    buf[i++] = '0';
    140c:	830aca93          	xori	s5,s5,-2000
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    1410:	00001997          	auipc	s3,0x1
    1414:	c3098993          	addi	s3,s3,-976 # 2040 <digits>
        if (!*s)
    1418:	00054783          	lbu	a5,0(a0)
    141c:	16078a63          	beqz	a5,1590 <printf+0x1ba>
    1420:	862a                	mv	a2,a0
        for (a = s; *s && *s != '%'; s++)
    1422:	19278163          	beq	a5,s2,15a4 <printf+0x1ce>
    1426:	00164783          	lbu	a5,1(a2)
    142a:	0605                	addi	a2,a2,1
    142c:	fbfd                	bnez	a5,1422 <printf+0x4c>
    142e:	84b2                	mv	s1,a2
        l = z - a;
    1430:	40a6043b          	subw	s0,a2,a0
    write(f, s, l);
    1434:	85aa                	mv	a1,a0
    1436:	8622                	mv	a2,s0
    1438:	4505                	li	a0,1
    143a:	0e3000ef          	jal	ra,1d1c <write>
        if (l)
    143e:	18041c63          	bnez	s0,15d6 <printf+0x200>
        if (s[1] == 0)
    1442:	0014c783          	lbu	a5,1(s1)
    1446:	14078563          	beqz	a5,1590 <printf+0x1ba>
        switch (s[1])
    144a:	1d478063          	beq	a5,s4,160a <printf+0x234>
    144e:	18fa6663          	bltu	s4,a5,15da <printf+0x204>
    1452:	06400713          	li	a4,100
    1456:	1ae78063          	beq	a5,a4,15f6 <printf+0x220>
    145a:	07000713          	li	a4,112
    145e:	1ce79963          	bne	a5,a4,1630 <printf+0x25a>
            printptr(va_arg(ap, uint64));
    1462:	6702                	ld	a4,0(sp)
    buf[i++] = '0';
    1464:	01511423          	sh	s5,8(sp)
    write(f, s, l);
    1468:	4649                	li	a2,18
            printptr(va_arg(ap, uint64));
    146a:	631c                	ld	a5,0(a4)
    146c:	0721                	addi	a4,a4,8
    146e:	e03a                	sd	a4,0(sp)
    for (j = 0; j < (sizeof(uint64) * 2); j++, x <<= 4)
    1470:	00479293          	slli	t0,a5,0x4
    1474:	00879f93          	slli	t6,a5,0x8
    1478:	00c79f13          	slli	t5,a5,0xc
    147c:	01079e93          	slli	t4,a5,0x10
    1480:	01479e13          	slli	t3,a5,0x14
    1484:	01879313          	slli	t1,a5,0x18
    1488:	01c79893          	slli	a7,a5,0x1c
    148c:	02479813          	slli	a6,a5,0x24
    1490:	02879513          	slli	a0,a5,0x28
    1494:	02c79593          	slli	a1,a5,0x2c
    1498:	03079693          	slli	a3,a5,0x30
    149c:	03479713          	slli	a4,a5,0x34
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    14a0:	03c7d413          	srli	s0,a5,0x3c
    14a4:	01c7d39b          	srliw	t2,a5,0x1c
    14a8:	03c2d293          	srli	t0,t0,0x3c
    14ac:	03cfdf93          	srli	t6,t6,0x3c
    14b0:	03cf5f13          	srli	t5,t5,0x3c
    14b4:	03cede93          	srli	t4,t4,0x3c
    14b8:	03ce5e13          	srli	t3,t3,0x3c
    14bc:	03c35313          	srli	t1,t1,0x3c
    14c0:	03c8d893          	srli	a7,a7,0x3c
    14c4:	03c85813          	srli	a6,a6,0x3c
    14c8:	9171                	srli	a0,a0,0x3c
    14ca:	91f1                	srli	a1,a1,0x3c
    14cc:	92f1                	srli	a3,a3,0x3c
    14ce:	9371                	srli	a4,a4,0x3c
    14d0:	96ce                	add	a3,a3,s3
    14d2:	974e                	add	a4,a4,s3
    14d4:	944e                	add	s0,s0,s3
    14d6:	92ce                	add	t0,t0,s3
    14d8:	9fce                	add	t6,t6,s3
    14da:	9f4e                	add	t5,t5,s3
    14dc:	9ece                	add	t4,t4,s3
    14de:	9e4e                	add	t3,t3,s3
    14e0:	934e                	add	t1,t1,s3
    14e2:	98ce                	add	a7,a7,s3
    14e4:	93ce                	add	t2,t2,s3
    14e6:	984e                	add	a6,a6,s3
    14e8:	954e                	add	a0,a0,s3
    14ea:	95ce                	add	a1,a1,s3
    14ec:	0006c083          	lbu	ra,0(a3)
    14f0:	0002c283          	lbu	t0,0(t0)
    14f4:	00074683          	lbu	a3,0(a4)
    14f8:	000fcf83          	lbu	t6,0(t6)
    14fc:	000f4f03          	lbu	t5,0(t5)
    1500:	000ece83          	lbu	t4,0(t4)
    1504:	000e4e03          	lbu	t3,0(t3)
    1508:	00034303          	lbu	t1,0(t1)
    150c:	0008c883          	lbu	a7,0(a7)
    1510:	0003c383          	lbu	t2,0(t2)
    1514:	00084803          	lbu	a6,0(a6)
    1518:	00054503          	lbu	a0,0(a0)
    151c:	0005c583          	lbu	a1,0(a1)
    1520:	00044403          	lbu	s0,0(s0)
    for (j = 0; j < (sizeof(uint64) * 2); j++, x <<= 4)
    1524:	03879713          	slli	a4,a5,0x38
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    1528:	9371                	srli	a4,a4,0x3c
    152a:	8bbd                	andi	a5,a5,15
    152c:	974e                	add	a4,a4,s3
    152e:	97ce                	add	a5,a5,s3
    1530:	005105a3          	sb	t0,11(sp)
    1534:	01f10623          	sb	t6,12(sp)
    1538:	01e106a3          	sb	t5,13(sp)
    153c:	01d10723          	sb	t4,14(sp)
    1540:	01c107a3          	sb	t3,15(sp)
    1544:	00610823          	sb	t1,16(sp)
    1548:	011108a3          	sb	a7,17(sp)
    154c:	00710923          	sb	t2,18(sp)
    1550:	010109a3          	sb	a6,19(sp)
    1554:	00a10a23          	sb	a0,20(sp)
    1558:	00b10aa3          	sb	a1,21(sp)
    155c:	00110b23          	sb	ra,22(sp)
    1560:	00d10ba3          	sb	a3,23(sp)
    1564:	00810523          	sb	s0,10(sp)
    1568:	00074703          	lbu	a4,0(a4)
    156c:	0007c783          	lbu	a5,0(a5)
    write(f, s, l);
    1570:	002c                	addi	a1,sp,8
    1572:	4505                	li	a0,1
        buf[i++] = digits[x >> (sizeof(uint64) * 8 - 4)];
    1574:	00e10c23          	sb	a4,24(sp)
    1578:	00f10ca3          	sb	a5,25(sp)
    buf[i] = 0;
    157c:	00010d23          	sb	zero,26(sp)
    write(f, s, l);
    1580:	79c000ef          	jal	ra,1d1c <write>
            // Print unknown % sequence to draw attention.
            putchar('%');
            putchar(s[1]);
            break;
        }
        s += 2;
    1584:	00248513          	addi	a0,s1,2
        if (!*s)
    1588:	00054783          	lbu	a5,0(a0)
    158c:	e8079ae3          	bnez	a5,1420 <printf+0x4a>
    }
    va_end(ap);
}
    1590:	70a6                	ld	ra,104(sp)
    1592:	7406                	ld	s0,96(sp)
    1594:	64e6                	ld	s1,88(sp)
    1596:	6946                	ld	s2,80(sp)
    1598:	69a6                	ld	s3,72(sp)
    159a:	6a06                	ld	s4,64(sp)
    159c:	7ae2                	ld	s5,56(sp)
    159e:	7b42                	ld	s6,48(sp)
    15a0:	614d                	addi	sp,sp,176
    15a2:	8082                	ret
        for (z = s; s[0] == '%' && s[1] == '%'; z++, s += 2)
    15a4:	00064783          	lbu	a5,0(a2)
    15a8:	84b2                	mv	s1,a2
    15aa:	01278963          	beq	a5,s2,15bc <printf+0x1e6>
    15ae:	b549                	j	1430 <printf+0x5a>
    15b0:	0024c783          	lbu	a5,2(s1)
    15b4:	0605                	addi	a2,a2,1
    15b6:	0489                	addi	s1,s1,2
    15b8:	e7279ce3          	bne	a5,s2,1430 <printf+0x5a>
    15bc:	0014c783          	lbu	a5,1(s1)
    15c0:	ff2788e3          	beq	a5,s2,15b0 <printf+0x1da>
        l = z - a;
    15c4:	40a6043b          	subw	s0,a2,a0
    write(f, s, l);
    15c8:	85aa                	mv	a1,a0
    15ca:	8622                	mv	a2,s0
    15cc:	4505                	li	a0,1
    15ce:	74e000ef          	jal	ra,1d1c <write>
        if (l)
    15d2:	e60408e3          	beqz	s0,1442 <printf+0x6c>
    15d6:	8526                	mv	a0,s1
    15d8:	b581                	j	1418 <printf+0x42>
        switch (s[1])
    15da:	07800713          	li	a4,120
    15de:	04e79963          	bne	a5,a4,1630 <printf+0x25a>
            printint(va_arg(ap, int), 16, 1);
    15e2:	6782                	ld	a5,0(sp)
    15e4:	45c1                	li	a1,16
    15e6:	4388                	lw	a0,0(a5)
    15e8:	07a1                	addi	a5,a5,8
    15ea:	e03e                	sd	a5,0(sp)
    15ec:	b71ff0ef          	jal	ra,115c <printint.constprop.0>
        s += 2;
    15f0:	00248513          	addi	a0,s1,2
    15f4:	bf51                	j	1588 <printf+0x1b2>
            printint(va_arg(ap, int), 10, 1);
    15f6:	6782                	ld	a5,0(sp)
    15f8:	45a9                	li	a1,10
    15fa:	4388                	lw	a0,0(a5)
    15fc:	07a1                	addi	a5,a5,8
    15fe:	e03e                	sd	a5,0(sp)
    1600:	b5dff0ef          	jal	ra,115c <printint.constprop.0>
        s += 2;
    1604:	00248513          	addi	a0,s1,2
    1608:	b741                	j	1588 <printf+0x1b2>
            if ((a = va_arg(ap, char *)) == 0)
    160a:	6782                	ld	a5,0(sp)
    160c:	6380                	ld	s0,0(a5)
    160e:	07a1                	addi	a5,a5,8
    1610:	e03e                	sd	a5,0(sp)
    1612:	c031                	beqz	s0,1656 <printf+0x280>
            l = strnlen(a, 200);
    1614:	0c800593          	li	a1,200
    1618:	8522                	mv	a0,s0
    161a:	40a000ef          	jal	ra,1a24 <strnlen>
    write(f, s, l);
    161e:	0005061b          	sext.w	a2,a0
    1622:	85a2                	mv	a1,s0
    1624:	4505                	li	a0,1
    1626:	6f6000ef          	jal	ra,1d1c <write>
        s += 2;
    162a:	00248513          	addi	a0,s1,2
    162e:	bfa9                	j	1588 <printf+0x1b2>
    return write(stdout, &byte, 1);
    1630:	4605                	li	a2,1
    1632:	002c                	addi	a1,sp,8
    1634:	4505                	li	a0,1
    char byte = c;
    1636:	01210423          	sb	s2,8(sp)
    return write(stdout, &byte, 1);
    163a:	6e2000ef          	jal	ra,1d1c <write>
    char byte = c;
    163e:	0014c783          	lbu	a5,1(s1)
    return write(stdout, &byte, 1);
    1642:	4605                	li	a2,1
    1644:	002c                	addi	a1,sp,8
    1646:	4505                	li	a0,1
    char byte = c;
    1648:	00f10423          	sb	a5,8(sp)
    return write(stdout, &byte, 1);
    164c:	6d0000ef          	jal	ra,1d1c <write>
        s += 2;
    1650:	00248513          	addi	a0,s1,2
    1654:	bf15                	j	1588 <printf+0x1b2>
                a = "(null)";
    1656:	845a                	mv	s0,s6
    1658:	bf75                	j	1614 <printf+0x23e>

000000000000165a <panic>:
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

void panic(char *m)
{
    165a:	1141                	addi	sp,sp,-16
    165c:	e406                	sd	ra,8(sp)
    puts(m);
    165e:	d57ff0ef          	jal	ra,13b4 <puts>
    exit(-100);
}
    1662:	60a2                	ld	ra,8(sp)
    exit(-100);
    1664:	f9c00513          	li	a0,-100
}
    1668:	0141                	addi	sp,sp,16
    exit(-100);
    166a:	a709                	j	1d6c <exit>

000000000000166c <isspace>:
#define HIGHS (ONES * (UCHAR_MAX / 2 + 1))
#define HASZERO(x) (((x)-ONES) & ~(x)&HIGHS)

int isspace(int c)
{
    return c == ' ' || (unsigned)c - '\t' < 5;
    166c:	02000793          	li	a5,32
    1670:	00f50663          	beq	a0,a5,167c <isspace+0x10>
    1674:	355d                	addiw	a0,a0,-9
    1676:	00553513          	sltiu	a0,a0,5
    167a:	8082                	ret
    167c:	4505                	li	a0,1
}
    167e:	8082                	ret

0000000000001680 <isdigit>:

int isdigit(int c)
{
    return (unsigned)c - '0' < 10;
    1680:	fd05051b          	addiw	a0,a0,-48
}
    1684:	00a53513          	sltiu	a0,a0,10
    1688:	8082                	ret

000000000000168a <atoi>:
    return c == ' ' || (unsigned)c - '\t' < 5;
    168a:	02000613          	li	a2,32
    168e:	4591                	li	a1,4

int atoi(const char *s)
{
    int n = 0, neg = 0;
    while (isspace(*s))
    1690:	00054703          	lbu	a4,0(a0)
    return c == ' ' || (unsigned)c - '\t' < 5;
    1694:	ff77069b          	addiw	a3,a4,-9
    1698:	04c70d63          	beq	a4,a2,16f2 <atoi+0x68>
    169c:	0007079b          	sext.w	a5,a4
    16a0:	04d5f963          	bgeu	a1,a3,16f2 <atoi+0x68>
        s++;
    switch (*s)
    16a4:	02b00693          	li	a3,43
    16a8:	04d70a63          	beq	a4,a3,16fc <atoi+0x72>
    16ac:	02d00693          	li	a3,45
    16b0:	06d70463          	beq	a4,a3,1718 <atoi+0x8e>
        neg = 1;
    case '+':
        s++;
    }
    /* Compute n as a negative number to avoid overflow on INT_MIN */
    while (isdigit(*s))
    16b4:	fd07859b          	addiw	a1,a5,-48
    16b8:	4625                	li	a2,9
    16ba:	873e                	mv	a4,a5
    16bc:	86aa                	mv	a3,a0
    int n = 0, neg = 0;
    16be:	4e01                	li	t3,0
    while (isdigit(*s))
    16c0:	04b66a63          	bltu	a2,a1,1714 <atoi+0x8a>
    int n = 0, neg = 0;
    16c4:	4501                	li	a0,0
    while (isdigit(*s))
    16c6:	4825                	li	a6,9
    16c8:	0016c603          	lbu	a2,1(a3)
        n = 10 * n - (*s++ - '0');
    16cc:	0025179b          	slliw	a5,a0,0x2
    16d0:	9d3d                	addw	a0,a0,a5
    16d2:	fd07031b          	addiw	t1,a4,-48
    16d6:	0015189b          	slliw	a7,a0,0x1
    while (isdigit(*s))
    16da:	fd06059b          	addiw	a1,a2,-48
        n = 10 * n - (*s++ - '0');
    16de:	0685                	addi	a3,a3,1
    16e0:	4068853b          	subw	a0,a7,t1
    while (isdigit(*s))
    16e4:	0006071b          	sext.w	a4,a2
    16e8:	feb870e3          	bgeu	a6,a1,16c8 <atoi+0x3e>
    return neg ? n : -n;
    16ec:	000e0563          	beqz	t3,16f6 <atoi+0x6c>
}
    16f0:	8082                	ret
        s++;
    16f2:	0505                	addi	a0,a0,1
    16f4:	bf71                	j	1690 <atoi+0x6>
    return neg ? n : -n;
    16f6:	4113053b          	subw	a0,t1,a7
    16fa:	8082                	ret
    while (isdigit(*s))
    16fc:	00154783          	lbu	a5,1(a0)
    1700:	4625                	li	a2,9
        s++;
    1702:	00150693          	addi	a3,a0,1
    while (isdigit(*s))
    1706:	fd07859b          	addiw	a1,a5,-48
    170a:	0007871b          	sext.w	a4,a5
    int n = 0, neg = 0;
    170e:	4e01                	li	t3,0
    while (isdigit(*s))
    1710:	fab67ae3          	bgeu	a2,a1,16c4 <atoi+0x3a>
    1714:	4501                	li	a0,0
}
    1716:	8082                	ret
    while (isdigit(*s))
    1718:	00154783          	lbu	a5,1(a0)
    171c:	4625                	li	a2,9
        s++;
    171e:	00150693          	addi	a3,a0,1
    while (isdigit(*s))
    1722:	fd07859b          	addiw	a1,a5,-48
    1726:	0007871b          	sext.w	a4,a5
    172a:	feb665e3          	bltu	a2,a1,1714 <atoi+0x8a>
        neg = 1;
    172e:	4e05                	li	t3,1
    1730:	bf51                	j	16c4 <atoi+0x3a>

0000000000001732 <memset>:

void *memset(void *dest, int c, size_t n)
{
    char *p = dest;
    for (int i = 0; i < n; ++i, *(p++) = c)
    1732:	16060d63          	beqz	a2,18ac <memset+0x17a>
    1736:	40a007b3          	neg	a5,a0
    173a:	8b9d                	andi	a5,a5,7
    173c:	00778713          	addi	a4,a5,7
    1740:	482d                	li	a6,11
    1742:	0ff5f593          	zext.b	a1,a1
    1746:	fff60693          	addi	a3,a2,-1
    174a:	17076263          	bltu	a4,a6,18ae <memset+0x17c>
    174e:	16e6ea63          	bltu	a3,a4,18c2 <memset+0x190>
    1752:	16078563          	beqz	a5,18bc <memset+0x18a>
    1756:	00b50023          	sb	a1,0(a0)
    175a:	4705                	li	a4,1
    175c:	00150e93          	addi	t4,a0,1
    1760:	14e78c63          	beq	a5,a4,18b8 <memset+0x186>
    1764:	00b500a3          	sb	a1,1(a0)
    1768:	4709                	li	a4,2
    176a:	00250e93          	addi	t4,a0,2
    176e:	14e78d63          	beq	a5,a4,18c8 <memset+0x196>
    1772:	00b50123          	sb	a1,2(a0)
    1776:	470d                	li	a4,3
    1778:	00350e93          	addi	t4,a0,3
    177c:	12e78b63          	beq	a5,a4,18b2 <memset+0x180>
    1780:	00b501a3          	sb	a1,3(a0)
    1784:	4711                	li	a4,4
    1786:	00450e93          	addi	t4,a0,4
    178a:	14e78163          	beq	a5,a4,18cc <memset+0x19a>
    178e:	00b50223          	sb	a1,4(a0)
    1792:	4715                	li	a4,5
    1794:	00550e93          	addi	t4,a0,5
    1798:	12e78c63          	beq	a5,a4,18d0 <memset+0x19e>
    179c:	00b502a3          	sb	a1,5(a0)
    17a0:	471d                	li	a4,7
    17a2:	00650e93          	addi	t4,a0,6
    17a6:	12e79763          	bne	a5,a4,18d4 <memset+0x1a2>
    17aa:	00750e93          	addi	t4,a0,7
    17ae:	00b50323          	sb	a1,6(a0)
    17b2:	4f1d                	li	t5,7
    17b4:	00859713          	slli	a4,a1,0x8
    17b8:	8f4d                	or	a4,a4,a1
    17ba:	01059e13          	slli	t3,a1,0x10
    17be:	01c76e33          	or	t3,a4,t3
    17c2:	01859313          	slli	t1,a1,0x18
    17c6:	006e6333          	or	t1,t3,t1
    17ca:	02059893          	slli	a7,a1,0x20
    17ce:	011368b3          	or	a7,t1,a7
    17d2:	02859813          	slli	a6,a1,0x28
    17d6:	40f60333          	sub	t1,a2,a5
    17da:	0108e833          	or	a6,a7,a6
    17de:	03059693          	slli	a3,a1,0x30
    17e2:	00d866b3          	or	a3,a6,a3
    17e6:	03859713          	slli	a4,a1,0x38
    17ea:	97aa                	add	a5,a5,a0
    17ec:	ff837813          	andi	a6,t1,-8
    17f0:	8f55                	or	a4,a4,a3
    17f2:	00f806b3          	add	a3,a6,a5
    17f6:	e398                	sd	a4,0(a5)
    17f8:	07a1                	addi	a5,a5,8
    17fa:	fed79ee3          	bne	a5,a3,17f6 <memset+0xc4>
    17fe:	ff837693          	andi	a3,t1,-8
    1802:	00de87b3          	add	a5,t4,a3
    1806:	01e6873b          	addw	a4,a3,t5
    180a:	0ad30663          	beq	t1,a3,18b6 <memset+0x184>
    180e:	00b78023          	sb	a1,0(a5)
    1812:	0017069b          	addiw	a3,a4,1
    1816:	08c6fb63          	bgeu	a3,a2,18ac <memset+0x17a>
    181a:	00b780a3          	sb	a1,1(a5)
    181e:	0027069b          	addiw	a3,a4,2
    1822:	08c6f563          	bgeu	a3,a2,18ac <memset+0x17a>
    1826:	00b78123          	sb	a1,2(a5)
    182a:	0037069b          	addiw	a3,a4,3
    182e:	06c6ff63          	bgeu	a3,a2,18ac <memset+0x17a>
    1832:	00b781a3          	sb	a1,3(a5)
    1836:	0047069b          	addiw	a3,a4,4
    183a:	06c6f963          	bgeu	a3,a2,18ac <memset+0x17a>
    183e:	00b78223          	sb	a1,4(a5)
    1842:	0057069b          	addiw	a3,a4,5
    1846:	06c6f363          	bgeu	a3,a2,18ac <memset+0x17a>
    184a:	00b782a3          	sb	a1,5(a5)
    184e:	0067069b          	addiw	a3,a4,6
    1852:	04c6fd63          	bgeu	a3,a2,18ac <memset+0x17a>
    1856:	00b78323          	sb	a1,6(a5)
    185a:	0077069b          	addiw	a3,a4,7
    185e:	04c6f763          	bgeu	a3,a2,18ac <memset+0x17a>
    1862:	00b783a3          	sb	a1,7(a5)
    1866:	0087069b          	addiw	a3,a4,8
    186a:	04c6f163          	bgeu	a3,a2,18ac <memset+0x17a>
    186e:	00b78423          	sb	a1,8(a5)
    1872:	0097069b          	addiw	a3,a4,9
    1876:	02c6fb63          	bgeu	a3,a2,18ac <memset+0x17a>
    187a:	00b784a3          	sb	a1,9(a5)
    187e:	00a7069b          	addiw	a3,a4,10
    1882:	02c6f563          	bgeu	a3,a2,18ac <memset+0x17a>
    1886:	00b78523          	sb	a1,10(a5)
    188a:	00b7069b          	addiw	a3,a4,11
    188e:	00c6ff63          	bgeu	a3,a2,18ac <memset+0x17a>
    1892:	00b785a3          	sb	a1,11(a5)
    1896:	00c7069b          	addiw	a3,a4,12
    189a:	00c6f963          	bgeu	a3,a2,18ac <memset+0x17a>
    189e:	00b78623          	sb	a1,12(a5)
    18a2:	2735                	addiw	a4,a4,13
    18a4:	00c77463          	bgeu	a4,a2,18ac <memset+0x17a>
    18a8:	00b786a3          	sb	a1,13(a5)
        ;
    return dest;
}
    18ac:	8082                	ret
    18ae:	472d                	li	a4,11
    18b0:	bd79                	j	174e <memset+0x1c>
    for (int i = 0; i < n; ++i, *(p++) = c)
    18b2:	4f0d                	li	t5,3
    18b4:	b701                	j	17b4 <memset+0x82>
    18b6:	8082                	ret
    18b8:	4f05                	li	t5,1
    18ba:	bded                	j	17b4 <memset+0x82>
    18bc:	8eaa                	mv	t4,a0
    18be:	4f01                	li	t5,0
    18c0:	bdd5                	j	17b4 <memset+0x82>
    18c2:	87aa                	mv	a5,a0
    18c4:	4701                	li	a4,0
    18c6:	b7a1                	j	180e <memset+0xdc>
    18c8:	4f09                	li	t5,2
    18ca:	b5ed                	j	17b4 <memset+0x82>
    18cc:	4f11                	li	t5,4
    18ce:	b5dd                	j	17b4 <memset+0x82>
    18d0:	4f15                	li	t5,5
    18d2:	b5cd                	j	17b4 <memset+0x82>
    18d4:	4f19                	li	t5,6
    18d6:	bdf9                	j	17b4 <memset+0x82>

00000000000018d8 <strcmp>:

int strcmp(const char *l, const char *r)
{
    for (; *l == *r && *l; l++, r++)
    18d8:	00054783          	lbu	a5,0(a0)
    18dc:	0005c703          	lbu	a4,0(a1)
    18e0:	00e79863          	bne	a5,a4,18f0 <strcmp+0x18>
    18e4:	0505                	addi	a0,a0,1
    18e6:	0585                	addi	a1,a1,1
    18e8:	fbe5                	bnez	a5,18d8 <strcmp>
    18ea:	4501                	li	a0,0
        ;
    return *(unsigned char *)l - *(unsigned char *)r;
}
    18ec:	9d19                	subw	a0,a0,a4
    18ee:	8082                	ret
    return *(unsigned char *)l - *(unsigned char *)r;
    18f0:	0007851b          	sext.w	a0,a5
    18f4:	bfe5                	j	18ec <strcmp+0x14>

00000000000018f6 <strncmp>:

int strncmp(const char *_l, const char *_r, size_t n)
{
    const unsigned char *l = (void *)_l, *r = (void *)_r;
    if (!n--)
    18f6:	ce05                	beqz	a2,192e <strncmp+0x38>
        return 0;
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    18f8:	00054703          	lbu	a4,0(a0)
    18fc:	0005c783          	lbu	a5,0(a1)
    1900:	cb0d                	beqz	a4,1932 <strncmp+0x3c>
    if (!n--)
    1902:	167d                	addi	a2,a2,-1
    1904:	00c506b3          	add	a3,a0,a2
    1908:	a819                	j	191e <strncmp+0x28>
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    190a:	00a68e63          	beq	a3,a0,1926 <strncmp+0x30>
    190e:	0505                	addi	a0,a0,1
    1910:	00e79b63          	bne	a5,a4,1926 <strncmp+0x30>
    1914:	00054703          	lbu	a4,0(a0)
        ;
    return *l - *r;
    1918:	0005c783          	lbu	a5,0(a1)
    for (; *l && *r && n && *l == *r; l++, r++, n--)
    191c:	cb19                	beqz	a4,1932 <strncmp+0x3c>
    191e:	0005c783          	lbu	a5,0(a1)
    1922:	0585                	addi	a1,a1,1
    1924:	f3fd                	bnez	a5,190a <strncmp+0x14>
    return *l - *r;
    1926:	0007051b          	sext.w	a0,a4
    192a:	9d1d                	subw	a0,a0,a5
    192c:	8082                	ret
        return 0;
    192e:	4501                	li	a0,0
}
    1930:	8082                	ret
    1932:	4501                	li	a0,0
    return *l - *r;
    1934:	9d1d                	subw	a0,a0,a5
    1936:	8082                	ret

0000000000001938 <strlen>:
size_t strlen(const char *s)
{
    const char *a = s;
    typedef size_t __attribute__((__may_alias__)) word;
    const word *w;
    for (; (uintptr_t)s % SS; s++)
    1938:	00757793          	andi	a5,a0,7
    193c:	cf89                	beqz	a5,1956 <strlen+0x1e>
    193e:	87aa                	mv	a5,a0
    1940:	a029                	j	194a <strlen+0x12>
    1942:	0785                	addi	a5,a5,1
    1944:	0077f713          	andi	a4,a5,7
    1948:	cb01                	beqz	a4,1958 <strlen+0x20>
        if (!*s)
    194a:	0007c703          	lbu	a4,0(a5)
    194e:	fb75                	bnez	a4,1942 <strlen+0xa>
    for (w = (const void *)s; !HASZERO(*w); w++)
        ;
    s = (const void *)w;
    for (; *s; s++)
        ;
    return s - a;
    1950:	40a78533          	sub	a0,a5,a0
}
    1954:	8082                	ret
    for (; (uintptr_t)s % SS; s++)
    1956:	87aa                	mv	a5,a0
    for (w = (const void *)s; !HASZERO(*w); w++)
    1958:	6394                	ld	a3,0(a5)
    195a:	00000597          	auipc	a1,0x0
    195e:	6b65b583          	ld	a1,1718(a1) # 2010 <__clone+0xbc>
    1962:	00000617          	auipc	a2,0x0
    1966:	6b663603          	ld	a2,1718(a2) # 2018 <__clone+0xc4>
    196a:	a019                	j	1970 <strlen+0x38>
    196c:	6794                	ld	a3,8(a5)
    196e:	07a1                	addi	a5,a5,8
    1970:	00b68733          	add	a4,a3,a1
    1974:	fff6c693          	not	a3,a3
    1978:	8f75                	and	a4,a4,a3
    197a:	8f71                	and	a4,a4,a2
    197c:	db65                	beqz	a4,196c <strlen+0x34>
    for (; *s; s++)
    197e:	0007c703          	lbu	a4,0(a5)
    1982:	d779                	beqz	a4,1950 <strlen+0x18>
    1984:	0017c703          	lbu	a4,1(a5)
    1988:	0785                	addi	a5,a5,1
    198a:	d379                	beqz	a4,1950 <strlen+0x18>
    198c:	0017c703          	lbu	a4,1(a5)
    1990:	0785                	addi	a5,a5,1
    1992:	fb6d                	bnez	a4,1984 <strlen+0x4c>
    1994:	bf75                	j	1950 <strlen+0x18>

0000000000001996 <memchr>:

void *memchr(const void *src, int c, size_t n)
{
    const unsigned char *s = src;
    c = (unsigned char)c;
    for (; ((uintptr_t)s & ALIGN) && n && *s != c; s++, n--)
    1996:	00757713          	andi	a4,a0,7
{
    199a:	87aa                	mv	a5,a0
    c = (unsigned char)c;
    199c:	0ff5f593          	zext.b	a1,a1
    for (; ((uintptr_t)s & ALIGN) && n && *s != c; s++, n--)
    19a0:	cb19                	beqz	a4,19b6 <memchr+0x20>
    19a2:	ce25                	beqz	a2,1a1a <memchr+0x84>
    19a4:	0007c703          	lbu	a4,0(a5)
    19a8:	04b70e63          	beq	a4,a1,1a04 <memchr+0x6e>
    19ac:	0785                	addi	a5,a5,1
    19ae:	0077f713          	andi	a4,a5,7
    19b2:	167d                	addi	a2,a2,-1
    19b4:	f77d                	bnez	a4,19a2 <memchr+0xc>
            ;
        s = (const void *)w;
    }
    for (; n && *s != c; s++, n--)
        ;
    return n ? (void *)s : 0;
    19b6:	4501                	li	a0,0
    if (n && *s != c)
    19b8:	c235                	beqz	a2,1a1c <memchr+0x86>
    19ba:	0007c703          	lbu	a4,0(a5)
    19be:	04b70363          	beq	a4,a1,1a04 <memchr+0x6e>
        size_t k = ONES * c;
    19c2:	00000517          	auipc	a0,0x0
    19c6:	65e53503          	ld	a0,1630(a0) # 2020 <__clone+0xcc>
        for (w = (const void *)s; n >= SS && !HASZERO(*w ^ k); w++, n -= SS)
    19ca:	471d                	li	a4,7
        size_t k = ONES * c;
    19cc:	02a58533          	mul	a0,a1,a0
        for (w = (const void *)s; n >= SS && !HASZERO(*w ^ k); w++, n -= SS)
    19d0:	02c77a63          	bgeu	a4,a2,1a04 <memchr+0x6e>
    19d4:	00000897          	auipc	a7,0x0
    19d8:	63c8b883          	ld	a7,1596(a7) # 2010 <__clone+0xbc>
    19dc:	00000817          	auipc	a6,0x0
    19e0:	63c83803          	ld	a6,1596(a6) # 2018 <__clone+0xc4>
    19e4:	431d                	li	t1,7
    19e6:	a029                	j	19f0 <memchr+0x5a>
    19e8:	1661                	addi	a2,a2,-8
    19ea:	07a1                	addi	a5,a5,8
    19ec:	02c37963          	bgeu	t1,a2,1a1e <memchr+0x88>
    19f0:	6398                	ld	a4,0(a5)
    19f2:	8f29                	xor	a4,a4,a0
    19f4:	011706b3          	add	a3,a4,a7
    19f8:	fff74713          	not	a4,a4
    19fc:	8f75                	and	a4,a4,a3
    19fe:	01077733          	and	a4,a4,a6
    1a02:	d37d                	beqz	a4,19e8 <memchr+0x52>
    1a04:	853e                	mv	a0,a5
    1a06:	97b2                	add	a5,a5,a2
    1a08:	a021                	j	1a10 <memchr+0x7a>
    for (; n && *s != c; s++, n--)
    1a0a:	0505                	addi	a0,a0,1
    1a0c:	00f50763          	beq	a0,a5,1a1a <memchr+0x84>
    1a10:	00054703          	lbu	a4,0(a0)
    1a14:	feb71be3          	bne	a4,a1,1a0a <memchr+0x74>
    1a18:	8082                	ret
    return n ? (void *)s : 0;
    1a1a:	4501                	li	a0,0
}
    1a1c:	8082                	ret
    return n ? (void *)s : 0;
    1a1e:	4501                	li	a0,0
    for (; n && *s != c; s++, n--)
    1a20:	f275                	bnez	a2,1a04 <memchr+0x6e>
}
    1a22:	8082                	ret

0000000000001a24 <strnlen>:

size_t strnlen(const char *s, size_t n)
{
    1a24:	1101                	addi	sp,sp,-32
    1a26:	e822                	sd	s0,16(sp)
    const char *p = memchr(s, 0, n);
    1a28:	862e                	mv	a2,a1
{
    1a2a:	842e                	mv	s0,a1
    const char *p = memchr(s, 0, n);
    1a2c:	4581                	li	a1,0
{
    1a2e:	e426                	sd	s1,8(sp)
    1a30:	ec06                	sd	ra,24(sp)
    1a32:	84aa                	mv	s1,a0
    const char *p = memchr(s, 0, n);
    1a34:	f63ff0ef          	jal	ra,1996 <memchr>
    return p ? p - s : n;
    1a38:	c519                	beqz	a0,1a46 <strnlen+0x22>
}
    1a3a:	60e2                	ld	ra,24(sp)
    1a3c:	6442                	ld	s0,16(sp)
    return p ? p - s : n;
    1a3e:	8d05                	sub	a0,a0,s1
}
    1a40:	64a2                	ld	s1,8(sp)
    1a42:	6105                	addi	sp,sp,32
    1a44:	8082                	ret
    1a46:	60e2                	ld	ra,24(sp)
    return p ? p - s : n;
    1a48:	8522                	mv	a0,s0
}
    1a4a:	6442                	ld	s0,16(sp)
    1a4c:	64a2                	ld	s1,8(sp)
    1a4e:	6105                	addi	sp,sp,32
    1a50:	8082                	ret

0000000000001a52 <strcpy>:
char *strcpy(char *restrict d, const char *s)
{
    typedef size_t __attribute__((__may_alias__)) word;
    word *wd;
    const word *ws;
    if ((uintptr_t)s % SS == (uintptr_t)d % SS)
    1a52:	00b547b3          	xor	a5,a0,a1
    1a56:	8b9d                	andi	a5,a5,7
    1a58:	eb95                	bnez	a5,1a8c <strcpy+0x3a>
    {
        for (; (uintptr_t)s % SS; s++, d++)
    1a5a:	0075f793          	andi	a5,a1,7
    1a5e:	e7b1                	bnez	a5,1aaa <strcpy+0x58>
            if (!(*d = *s))
                return d;
        wd = (void *)d;
        ws = (const void *)s;
        for (; !HASZERO(*ws); *wd++ = *ws++)
    1a60:	6198                	ld	a4,0(a1)
    1a62:	00000617          	auipc	a2,0x0
    1a66:	5ae63603          	ld	a2,1454(a2) # 2010 <__clone+0xbc>
    1a6a:	00000817          	auipc	a6,0x0
    1a6e:	5ae83803          	ld	a6,1454(a6) # 2018 <__clone+0xc4>
    1a72:	a029                	j	1a7c <strcpy+0x2a>
    1a74:	e118                	sd	a4,0(a0)
    1a76:	6598                	ld	a4,8(a1)
    1a78:	05a1                	addi	a1,a1,8
    1a7a:	0521                	addi	a0,a0,8
    1a7c:	00c707b3          	add	a5,a4,a2
    1a80:	fff74693          	not	a3,a4
    1a84:	8ff5                	and	a5,a5,a3
    1a86:	0107f7b3          	and	a5,a5,a6
    1a8a:	d7ed                	beqz	a5,1a74 <strcpy+0x22>
            ;
        d = (void *)wd;
        s = (const void *)ws;
    }
    for (; (*d = *s); s++, d++)
    1a8c:	0005c783          	lbu	a5,0(a1)
    1a90:	00f50023          	sb	a5,0(a0)
    1a94:	c785                	beqz	a5,1abc <strcpy+0x6a>
    1a96:	0015c783          	lbu	a5,1(a1)
    1a9a:	0505                	addi	a0,a0,1
    1a9c:	0585                	addi	a1,a1,1
    1a9e:	00f50023          	sb	a5,0(a0)
    1aa2:	fbf5                	bnez	a5,1a96 <strcpy+0x44>
        ;
    return d;
}
    1aa4:	8082                	ret
        for (; (uintptr_t)s % SS; s++, d++)
    1aa6:	0505                	addi	a0,a0,1
    1aa8:	df45                	beqz	a4,1a60 <strcpy+0xe>
            if (!(*d = *s))
    1aaa:	0005c783          	lbu	a5,0(a1)
        for (; (uintptr_t)s % SS; s++, d++)
    1aae:	0585                	addi	a1,a1,1
    1ab0:	0075f713          	andi	a4,a1,7
            if (!(*d = *s))
    1ab4:	00f50023          	sb	a5,0(a0)
    1ab8:	f7fd                	bnez	a5,1aa6 <strcpy+0x54>
}
    1aba:	8082                	ret
    1abc:	8082                	ret

0000000000001abe <strncpy>:
char *strncpy(char *restrict d, const char *s, size_t n)
{
    typedef size_t __attribute__((__may_alias__)) word;
    word *wd;
    const word *ws;
    if (((uintptr_t)s & ALIGN) == ((uintptr_t)d & ALIGN))
    1abe:	00b547b3          	xor	a5,a0,a1
    1ac2:	8b9d                	andi	a5,a5,7
    1ac4:	1a079863          	bnez	a5,1c74 <strncpy+0x1b6>
    {
        for (; ((uintptr_t)s & ALIGN) && n && (*d = *s); n--, s++, d++)
    1ac8:	0075f793          	andi	a5,a1,7
    1acc:	16078463          	beqz	a5,1c34 <strncpy+0x176>
    1ad0:	ea01                	bnez	a2,1ae0 <strncpy+0x22>
    1ad2:	a421                	j	1cda <strncpy+0x21c>
    1ad4:	167d                	addi	a2,a2,-1
    1ad6:	0505                	addi	a0,a0,1
    1ad8:	14070e63          	beqz	a4,1c34 <strncpy+0x176>
    1adc:	1a060863          	beqz	a2,1c8c <strncpy+0x1ce>
    1ae0:	0005c783          	lbu	a5,0(a1)
    1ae4:	0585                	addi	a1,a1,1
    1ae6:	0075f713          	andi	a4,a1,7
    1aea:	00f50023          	sb	a5,0(a0)
    1aee:	f3fd                	bnez	a5,1ad4 <strncpy+0x16>
    1af0:	4805                	li	a6,1
    1af2:	1a061863          	bnez	a2,1ca2 <strncpy+0x1e4>
    1af6:	40a007b3          	neg	a5,a0
    1afa:	8b9d                	andi	a5,a5,7
    1afc:	4681                	li	a3,0
    1afe:	18061a63          	bnez	a2,1c92 <strncpy+0x1d4>
    1b02:	00778713          	addi	a4,a5,7
    1b06:	45ad                	li	a1,11
    1b08:	18b76363          	bltu	a4,a1,1c8e <strncpy+0x1d0>
    1b0c:	1ae6eb63          	bltu	a3,a4,1cc2 <strncpy+0x204>
    1b10:	1a078363          	beqz	a5,1cb6 <strncpy+0x1f8>
    for (int i = 0; i < n; ++i, *(p++) = c)
    1b14:	00050023          	sb	zero,0(a0)
    1b18:	4685                	li	a3,1
    1b1a:	00150713          	addi	a4,a0,1
    1b1e:	18d78f63          	beq	a5,a3,1cbc <strncpy+0x1fe>
    1b22:	000500a3          	sb	zero,1(a0)
    1b26:	4689                	li	a3,2
    1b28:	00250713          	addi	a4,a0,2
    1b2c:	18d78e63          	beq	a5,a3,1cc8 <strncpy+0x20a>
    1b30:	00050123          	sb	zero,2(a0)
    1b34:	468d                	li	a3,3
    1b36:	00350713          	addi	a4,a0,3
    1b3a:	16d78c63          	beq	a5,a3,1cb2 <strncpy+0x1f4>
    1b3e:	000501a3          	sb	zero,3(a0)
    1b42:	4691                	li	a3,4
    1b44:	00450713          	addi	a4,a0,4
    1b48:	18d78263          	beq	a5,a3,1ccc <strncpy+0x20e>
    1b4c:	00050223          	sb	zero,4(a0)
    1b50:	4695                	li	a3,5
    1b52:	00550713          	addi	a4,a0,5
    1b56:	16d78d63          	beq	a5,a3,1cd0 <strncpy+0x212>
    1b5a:	000502a3          	sb	zero,5(a0)
    1b5e:	469d                	li	a3,7
    1b60:	00650713          	addi	a4,a0,6
    1b64:	16d79863          	bne	a5,a3,1cd4 <strncpy+0x216>
    1b68:	00750713          	addi	a4,a0,7
    1b6c:	00050323          	sb	zero,6(a0)
    1b70:	40f80833          	sub	a6,a6,a5
    1b74:	ff887593          	andi	a1,a6,-8
    1b78:	97aa                	add	a5,a5,a0
    1b7a:	95be                	add	a1,a1,a5
    1b7c:	0007b023          	sd	zero,0(a5)
    1b80:	07a1                	addi	a5,a5,8
    1b82:	feb79de3          	bne	a5,a1,1b7c <strncpy+0xbe>
    1b86:	ff887593          	andi	a1,a6,-8
    1b8a:	9ead                	addw	a3,a3,a1
    1b8c:	00b707b3          	add	a5,a4,a1
    1b90:	12b80863          	beq	a6,a1,1cc0 <strncpy+0x202>
    1b94:	00078023          	sb	zero,0(a5)
    1b98:	0016871b          	addiw	a4,a3,1
    1b9c:	0ec77863          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1ba0:	000780a3          	sb	zero,1(a5)
    1ba4:	0026871b          	addiw	a4,a3,2
    1ba8:	0ec77263          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1bac:	00078123          	sb	zero,2(a5)
    1bb0:	0036871b          	addiw	a4,a3,3
    1bb4:	0cc77c63          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1bb8:	000781a3          	sb	zero,3(a5)
    1bbc:	0046871b          	addiw	a4,a3,4
    1bc0:	0cc77663          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1bc4:	00078223          	sb	zero,4(a5)
    1bc8:	0056871b          	addiw	a4,a3,5
    1bcc:	0cc77063          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1bd0:	000782a3          	sb	zero,5(a5)
    1bd4:	0066871b          	addiw	a4,a3,6
    1bd8:	0ac77a63          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1bdc:	00078323          	sb	zero,6(a5)
    1be0:	0076871b          	addiw	a4,a3,7
    1be4:	0ac77463          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1be8:	000783a3          	sb	zero,7(a5)
    1bec:	0086871b          	addiw	a4,a3,8
    1bf0:	08c77e63          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1bf4:	00078423          	sb	zero,8(a5)
    1bf8:	0096871b          	addiw	a4,a3,9
    1bfc:	08c77863          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1c00:	000784a3          	sb	zero,9(a5)
    1c04:	00a6871b          	addiw	a4,a3,10
    1c08:	08c77263          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1c0c:	00078523          	sb	zero,10(a5)
    1c10:	00b6871b          	addiw	a4,a3,11
    1c14:	06c77c63          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1c18:	000785a3          	sb	zero,11(a5)
    1c1c:	00c6871b          	addiw	a4,a3,12
    1c20:	06c77663          	bgeu	a4,a2,1c8c <strncpy+0x1ce>
    1c24:	00078623          	sb	zero,12(a5)
    1c28:	26b5                	addiw	a3,a3,13
    1c2a:	06c6f163          	bgeu	a3,a2,1c8c <strncpy+0x1ce>
    1c2e:	000786a3          	sb	zero,13(a5)
    1c32:	8082                	ret
            ;
        if (!n || !*s)
    1c34:	c645                	beqz	a2,1cdc <strncpy+0x21e>
    1c36:	0005c783          	lbu	a5,0(a1)
    1c3a:	ea078be3          	beqz	a5,1af0 <strncpy+0x32>
            goto tail;
        wd = (void *)d;
        ws = (const void *)s;
        for (; n >= sizeof(size_t) && !HASZERO(*ws); n -= sizeof(size_t), ws++, wd++)
    1c3e:	479d                	li	a5,7
    1c40:	02c7ff63          	bgeu	a5,a2,1c7e <strncpy+0x1c0>
    1c44:	00000897          	auipc	a7,0x0
    1c48:	3cc8b883          	ld	a7,972(a7) # 2010 <__clone+0xbc>
    1c4c:	00000817          	auipc	a6,0x0
    1c50:	3cc83803          	ld	a6,972(a6) # 2018 <__clone+0xc4>
    1c54:	431d                	li	t1,7
    1c56:	6198                	ld	a4,0(a1)
    1c58:	011707b3          	add	a5,a4,a7
    1c5c:	fff74693          	not	a3,a4
    1c60:	8ff5                	and	a5,a5,a3
    1c62:	0107f7b3          	and	a5,a5,a6
    1c66:	ef81                	bnez	a5,1c7e <strncpy+0x1c0>
            *wd = *ws;
    1c68:	e118                	sd	a4,0(a0)
        for (; n >= sizeof(size_t) && !HASZERO(*ws); n -= sizeof(size_t), ws++, wd++)
    1c6a:	1661                	addi	a2,a2,-8
    1c6c:	05a1                	addi	a1,a1,8
    1c6e:	0521                	addi	a0,a0,8
    1c70:	fec363e3          	bltu	t1,a2,1c56 <strncpy+0x198>
        d = (void *)wd;
        s = (const void *)ws;
    }
    for (; n && (*d = *s); n--, s++, d++)
    1c74:	e609                	bnez	a2,1c7e <strncpy+0x1c0>
    1c76:	a08d                	j	1cd8 <strncpy+0x21a>
    1c78:	167d                	addi	a2,a2,-1
    1c7a:	0505                	addi	a0,a0,1
    1c7c:	ca01                	beqz	a2,1c8c <strncpy+0x1ce>
    1c7e:	0005c783          	lbu	a5,0(a1)
    1c82:	0585                	addi	a1,a1,1
    1c84:	00f50023          	sb	a5,0(a0)
    1c88:	fbe5                	bnez	a5,1c78 <strncpy+0x1ba>
        ;
tail:
    1c8a:	b59d                	j	1af0 <strncpy+0x32>
    memset(d, 0, n);
    return d;
}
    1c8c:	8082                	ret
    1c8e:	472d                	li	a4,11
    1c90:	bdb5                	j	1b0c <strncpy+0x4e>
    1c92:	00778713          	addi	a4,a5,7
    1c96:	45ad                	li	a1,11
    1c98:	fff60693          	addi	a3,a2,-1
    1c9c:	e6b778e3          	bgeu	a4,a1,1b0c <strncpy+0x4e>
    1ca0:	b7fd                	j	1c8e <strncpy+0x1d0>
    1ca2:	40a007b3          	neg	a5,a0
    1ca6:	8832                	mv	a6,a2
    1ca8:	8b9d                	andi	a5,a5,7
    1caa:	4681                	li	a3,0
    1cac:	e4060be3          	beqz	a2,1b02 <strncpy+0x44>
    1cb0:	b7cd                	j	1c92 <strncpy+0x1d4>
    for (int i = 0; i < n; ++i, *(p++) = c)
    1cb2:	468d                	li	a3,3
    1cb4:	bd75                	j	1b70 <strncpy+0xb2>
    1cb6:	872a                	mv	a4,a0
    1cb8:	4681                	li	a3,0
    1cba:	bd5d                	j	1b70 <strncpy+0xb2>
    1cbc:	4685                	li	a3,1
    1cbe:	bd4d                	j	1b70 <strncpy+0xb2>
    1cc0:	8082                	ret
    1cc2:	87aa                	mv	a5,a0
    1cc4:	4681                	li	a3,0
    1cc6:	b5f9                	j	1b94 <strncpy+0xd6>
    1cc8:	4689                	li	a3,2
    1cca:	b55d                	j	1b70 <strncpy+0xb2>
    1ccc:	4691                	li	a3,4
    1cce:	b54d                	j	1b70 <strncpy+0xb2>
    1cd0:	4695                	li	a3,5
    1cd2:	bd79                	j	1b70 <strncpy+0xb2>
    1cd4:	4699                	li	a3,6
    1cd6:	bd69                	j	1b70 <strncpy+0xb2>
    1cd8:	8082                	ret
    1cda:	8082                	ret
    1cdc:	8082                	ret

0000000000001cde <open>:
#include <unistd.h>

#include "syscall.h"

int open(const char *path, int flags)
{
    1cde:	87aa                	mv	a5,a0
    1ce0:	862e                	mv	a2,a1
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
    register long a7 __asm__("a7") = n;
    1ce2:	03800893          	li	a7,56
    register long a0 __asm__("a0") = a;
    1ce6:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1cea:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    1cec:	4689                	li	a3,2
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1cee:	00000073          	ecall
    return syscall(SYS_openat, AT_FDCWD, path, flags, O_RDWR);
}
    1cf2:	2501                	sext.w	a0,a0
    1cf4:	8082                	ret

0000000000001cf6 <openat>:
    register long a7 __asm__("a7") = n;
    1cf6:	03800893          	li	a7,56
    register long a3 __asm__("a3") = d;
    1cfa:	18000693          	li	a3,384
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1cfe:	00000073          	ecall

int openat(int dirfd,const char *path, int flags)
{
    return syscall(SYS_openat, dirfd, path, flags, 0600);
}
    1d02:	2501                	sext.w	a0,a0
    1d04:	8082                	ret

0000000000001d06 <close>:
    register long a7 __asm__("a7") = n;
    1d06:	03900893          	li	a7,57
    __asm_syscall("r"(a7), "0"(a0))
    1d0a:	00000073          	ecall

int close(int fd)
{
    return syscall(SYS_close, fd);
}
    1d0e:	2501                	sext.w	a0,a0
    1d10:	8082                	ret

0000000000001d12 <read>:
    register long a7 __asm__("a7") = n;
    1d12:	03f00893          	li	a7,63
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1d16:	00000073          	ecall

ssize_t read(int fd, void *buf, size_t len)
{
    return syscall(SYS_read, fd, buf, len);
}
    1d1a:	8082                	ret

0000000000001d1c <write>:
    register long a7 __asm__("a7") = n;
    1d1c:	04000893          	li	a7,64
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1d20:	00000073          	ecall

ssize_t write(int fd, const void *buf, size_t len)
{
    return syscall(SYS_write, fd, buf, len);
}
    1d24:	8082                	ret

0000000000001d26 <getpid>:
    register long a7 __asm__("a7") = n;
    1d26:	0ac00893          	li	a7,172
    __asm_syscall("r"(a7))
    1d2a:	00000073          	ecall

pid_t getpid(void)
{
    return syscall(SYS_getpid);
}
    1d2e:	2501                	sext.w	a0,a0
    1d30:	8082                	ret

0000000000001d32 <getppid>:
    register long a7 __asm__("a7") = n;
    1d32:	0ad00893          	li	a7,173
    __asm_syscall("r"(a7))
    1d36:	00000073          	ecall

pid_t getppid(void)
{
    return syscall(SYS_getppid);
}
    1d3a:	2501                	sext.w	a0,a0
    1d3c:	8082                	ret

0000000000001d3e <sched_yield>:
    register long a7 __asm__("a7") = n;
    1d3e:	07c00893          	li	a7,124
    __asm_syscall("r"(a7))
    1d42:	00000073          	ecall

int sched_yield(void)
{
    return syscall(SYS_sched_yield);
}
    1d46:	2501                	sext.w	a0,a0
    1d48:	8082                	ret

0000000000001d4a <fork>:
    register long a7 __asm__("a7") = n;
    1d4a:	0dc00893          	li	a7,220
    register long a0 __asm__("a0") = a;
    1d4e:	4545                	li	a0,17
    register long a1 __asm__("a1") = b;
    1d50:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1d52:	00000073          	ecall

pid_t fork(void)
{
    return syscall(SYS_clone, SIGCHLD, 0);
}
    1d56:	2501                	sext.w	a0,a0
    1d58:	8082                	ret

0000000000001d5a <clone>:

pid_t clone(int (*fn)(void *arg), void *arg, void *stack, size_t stack_size, unsigned long flags)
{
    1d5a:	85b2                	mv	a1,a2
    1d5c:	863a                	mv	a2,a4
    if (stack)
    1d5e:	c191                	beqz	a1,1d62 <clone+0x8>
	stack += stack_size;
    1d60:	95b6                	add	a1,a1,a3

    return __clone(fn, stack, flags, NULL, NULL, NULL);
    1d62:	4781                	li	a5,0
    1d64:	4701                	li	a4,0
    1d66:	4681                	li	a3,0
    1d68:	2601                	sext.w	a2,a2
    1d6a:	a2ed                	j	1f54 <__clone>

0000000000001d6c <exit>:
    register long a7 __asm__("a7") = n;
    1d6c:	05d00893          	li	a7,93
    __asm_syscall("r"(a7), "0"(a0))
    1d70:	00000073          	ecall
    //return syscall(SYS_clone, fn, stack, flags, NULL, NULL, NULL);
}
void exit(int code)
{
    syscall(SYS_exit, code);
}
    1d74:	8082                	ret

0000000000001d76 <waitpid>:
    register long a7 __asm__("a7") = n;
    1d76:	10400893          	li	a7,260
    register long a3 __asm__("a3") = d;
    1d7a:	4681                	li	a3,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1d7c:	00000073          	ecall

int waitpid(int pid, int *code, int options)
{
    return syscall(SYS_wait4, pid, code, options, 0);
}
    1d80:	2501                	sext.w	a0,a0
    1d82:	8082                	ret

0000000000001d84 <exec>:
    register long a7 __asm__("a7") = n;
    1d84:	0dd00893          	li	a7,221
    __asm_syscall("r"(a7), "0"(a0))
    1d88:	00000073          	ecall

int exec(char *name)
{
    return syscall(SYS_execve, name);
}
    1d8c:	2501                	sext.w	a0,a0
    1d8e:	8082                	ret

0000000000001d90 <execve>:
    register long a7 __asm__("a7") = n;
    1d90:	0dd00893          	li	a7,221
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1d94:	00000073          	ecall

int execve(const char *name, char *const argv[], char *const argp[])
{
    return syscall(SYS_execve, name, argv, argp);
}
    1d98:	2501                	sext.w	a0,a0
    1d9a:	8082                	ret

0000000000001d9c <times>:
    register long a7 __asm__("a7") = n;
    1d9c:	09900893          	li	a7,153
    __asm_syscall("r"(a7), "0"(a0))
    1da0:	00000073          	ecall

int times(void *mytimes)
{
	return syscall(SYS_times, mytimes);
}
    1da4:	2501                	sext.w	a0,a0
    1da6:	8082                	ret

0000000000001da8 <get_time>:

int64 get_time()
{
    1da8:	1141                	addi	sp,sp,-16
    register long a7 __asm__("a7") = n;
    1daa:	0a900893          	li	a7,169
    register long a0 __asm__("a0") = a;
    1dae:	850a                	mv	a0,sp
    register long a1 __asm__("a1") = b;
    1db0:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1db2:	00000073          	ecall
    TimeVal time;
    int err = sys_get_time(&time, 0);
    if (err == 0)
    1db6:	2501                	sext.w	a0,a0
    1db8:	ed09                	bnez	a0,1dd2 <get_time+0x2a>
    {
        return ((time.sec & 0xffff) * 1000 + time.usec / 1000);
    1dba:	67a2                	ld	a5,8(sp)
    1dbc:	3e800713          	li	a4,1000
    1dc0:	00015503          	lhu	a0,0(sp)
    1dc4:	02e7d7b3          	divu	a5,a5,a4
    1dc8:	02e50533          	mul	a0,a0,a4
    1dcc:	953e                	add	a0,a0,a5
    }
    else
    {
        return -1;
    }
}
    1dce:	0141                	addi	sp,sp,16
    1dd0:	8082                	ret
        return -1;
    1dd2:	557d                	li	a0,-1
    1dd4:	bfed                	j	1dce <get_time+0x26>

0000000000001dd6 <sys_get_time>:
    register long a7 __asm__("a7") = n;
    1dd6:	0a900893          	li	a7,169
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1dda:	00000073          	ecall

int sys_get_time(TimeVal *ts, int tz)
{
    return syscall(SYS_gettimeofday, ts, tz);
}
    1dde:	2501                	sext.w	a0,a0
    1de0:	8082                	ret

0000000000001de2 <time>:
    register long a7 __asm__("a7") = n;
    1de2:	42600893          	li	a7,1062
    __asm_syscall("r"(a7), "0"(a0))
    1de6:	00000073          	ecall

int time(unsigned long *tloc)
{
    return syscall(SYS_time, tloc);
}
    1dea:	2501                	sext.w	a0,a0
    1dec:	8082                	ret

0000000000001dee <sleep>:

int sleep(unsigned long long time)
{
    1dee:	1141                	addi	sp,sp,-16
    TimeVal tv = {.sec = time, .usec = 0};
    1df0:	e02a                	sd	a0,0(sp)
    register long a0 __asm__("a0") = a;
    1df2:	850a                	mv	a0,sp
    1df4:	e402                	sd	zero,8(sp)
    register long a7 __asm__("a7") = n;
    1df6:	06500893          	li	a7,101
    register long a1 __asm__("a1") = b;
    1dfa:	85aa                	mv	a1,a0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1dfc:	00000073          	ecall
    if (syscall(SYS_nanosleep, &tv, &tv)) return tv.sec;
    1e00:	e501                	bnez	a0,1e08 <sleep+0x1a>
    return 0;
    1e02:	4501                	li	a0,0
}
    1e04:	0141                	addi	sp,sp,16
    1e06:	8082                	ret
    if (syscall(SYS_nanosleep, &tv, &tv)) return tv.sec;
    1e08:	4502                	lw	a0,0(sp)
}
    1e0a:	0141                	addi	sp,sp,16
    1e0c:	8082                	ret

0000000000001e0e <set_priority>:
    register long a7 __asm__("a7") = n;
    1e0e:	08c00893          	li	a7,140
    __asm_syscall("r"(a7), "0"(a0))
    1e12:	00000073          	ecall

int set_priority(int prio)
{
    return syscall(SYS_setpriority, prio);
}
    1e16:	2501                	sext.w	a0,a0
    1e18:	8082                	ret

0000000000001e1a <mmap>:
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
    register long a7 __asm__("a7") = n;
    1e1a:	0de00893          	li	a7,222
    register long a1 __asm__("a1") = b;
    register long a2 __asm__("a2") = c;
    register long a3 __asm__("a3") = d;
    register long a4 __asm__("a4") = e;
    register long a5 __asm__("a5") = f;
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5))
    1e1e:	00000073          	ecall

void *mmap(void *start, size_t len, int prot, int flags, int fd, off_t off)
{
    return syscall(SYS_mmap, start, len, prot, flags, fd, off);
}
    1e22:	8082                	ret

0000000000001e24 <munmap>:
    register long a7 __asm__("a7") = n;
    1e24:	0d700893          	li	a7,215
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e28:	00000073          	ecall

int munmap(void *start, size_t len)
{
    return syscall(SYS_munmap, start, len);
}
    1e2c:	2501                	sext.w	a0,a0
    1e2e:	8082                	ret

0000000000001e30 <wait>:

int wait(int *code)
{
    1e30:	85aa                	mv	a1,a0
    register long a7 __asm__("a7") = n;
    1e32:	10400893          	li	a7,260
    register long a0 __asm__("a0") = a;
    1e36:	557d                	li	a0,-1
    register long a2 __asm__("a2") = c;
    1e38:	4601                	li	a2,0
    register long a3 __asm__("a3") = d;
    1e3a:	4681                	li	a3,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3))
    1e3c:	00000073          	ecall
    return waitpid((int)-1, code, 0);
}
    1e40:	2501                	sext.w	a0,a0
    1e42:	8082                	ret

0000000000001e44 <spawn>:
    register long a7 __asm__("a7") = n;
    1e44:	19000893          	li	a7,400
    __asm_syscall("r"(a7), "0"(a0))
    1e48:	00000073          	ecall

int spawn(char *file)
{
    return syscall(SYS_spawn, file);
}
    1e4c:	2501                	sext.w	a0,a0
    1e4e:	8082                	ret

0000000000001e50 <mailread>:
    register long a7 __asm__("a7") = n;
    1e50:	19100893          	li	a7,401
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e54:	00000073          	ecall

int mailread(void *buf, int len)
{
    return syscall(SYS_mailread, buf, len);
}
    1e58:	2501                	sext.w	a0,a0
    1e5a:	8082                	ret

0000000000001e5c <mailwrite>:
    register long a7 __asm__("a7") = n;
    1e5c:	19200893          	li	a7,402
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e60:	00000073          	ecall

int mailwrite(int pid, void *buf, int len)
{
    return syscall(SYS_mailwrite, pid, buf, len);
}
    1e64:	2501                	sext.w	a0,a0
    1e66:	8082                	ret

0000000000001e68 <fstat>:
    register long a7 __asm__("a7") = n;
    1e68:	05000893          	li	a7,80
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1e6c:	00000073          	ecall

int fstat(int fd, struct kstat *st)
{
    return syscall(SYS_fstat, fd, st);
}
    1e70:	2501                	sext.w	a0,a0
    1e72:	8082                	ret

0000000000001e74 <sys_linkat>:
    register long a4 __asm__("a4") = e;
    1e74:	1702                	slli	a4,a4,0x20
    register long a7 __asm__("a7") = n;
    1e76:	02500893          	li	a7,37
    register long a4 __asm__("a4") = e;
    1e7a:	9301                	srli	a4,a4,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1e7c:	00000073          	ecall

int sys_linkat(int olddirfd, char *oldpath, int newdirfd, char *newpath, unsigned int flags)
{
    return syscall(SYS_linkat, olddirfd, oldpath, newdirfd, newpath, flags);
}
    1e80:	2501                	sext.w	a0,a0
    1e82:	8082                	ret

0000000000001e84 <sys_unlinkat>:
    register long a2 __asm__("a2") = c;
    1e84:	1602                	slli	a2,a2,0x20
    register long a7 __asm__("a7") = n;
    1e86:	02300893          	li	a7,35
    register long a2 __asm__("a2") = c;
    1e8a:	9201                	srli	a2,a2,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1e8c:	00000073          	ecall

int sys_unlinkat(int dirfd, char *path, unsigned int flags)
{
    return syscall(SYS_unlinkat, dirfd, path, flags);
}
    1e90:	2501                	sext.w	a0,a0
    1e92:	8082                	ret

0000000000001e94 <link>:

int link(char *old_path, char *new_path)
{
    1e94:	87aa                	mv	a5,a0
    1e96:	86ae                	mv	a3,a1
    register long a7 __asm__("a7") = n;
    1e98:	02500893          	li	a7,37
    register long a0 __asm__("a0") = a;
    1e9c:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1ea0:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    1ea2:	f9c00613          	li	a2,-100
    register long a4 __asm__("a4") = e;
    1ea6:	4701                	li	a4,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1ea8:	00000073          	ecall
    return sys_linkat(AT_FDCWD, old_path, AT_FDCWD, new_path, 0);
}
    1eac:	2501                	sext.w	a0,a0
    1eae:	8082                	ret

0000000000001eb0 <unlink>:

int unlink(char *path)
{
    1eb0:	85aa                	mv	a1,a0
    register long a7 __asm__("a7") = n;
    1eb2:	02300893          	li	a7,35
    register long a0 __asm__("a0") = a;
    1eb6:	f9c00513          	li	a0,-100
    register long a2 __asm__("a2") = c;
    1eba:	4601                	li	a2,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1ebc:	00000073          	ecall
    return sys_unlinkat(AT_FDCWD, path, 0);
}
    1ec0:	2501                	sext.w	a0,a0
    1ec2:	8082                	ret

0000000000001ec4 <uname>:
    register long a7 __asm__("a7") = n;
    1ec4:	0a000893          	li	a7,160
    __asm_syscall("r"(a7), "0"(a0))
    1ec8:	00000073          	ecall

int uname(void *buf)
{
    return syscall(SYS_uname, buf);
}
    1ecc:	2501                	sext.w	a0,a0
    1ece:	8082                	ret

0000000000001ed0 <brk>:
    register long a7 __asm__("a7") = n;
    1ed0:	0d600893          	li	a7,214
    __asm_syscall("r"(a7), "0"(a0))
    1ed4:	00000073          	ecall

int brk(void *addr)
{
    return syscall(SYS_brk, addr);
}
    1ed8:	2501                	sext.w	a0,a0
    1eda:	8082                	ret

0000000000001edc <getcwd>:
    register long a7 __asm__("a7") = n;
    1edc:	48c5                	li	a7,17
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1ede:	00000073          	ecall

char *getcwd(char *buf, size_t size){
    return syscall(SYS_getcwd, buf, size);
}
    1ee2:	8082                	ret

0000000000001ee4 <chdir>:
    register long a7 __asm__("a7") = n;
    1ee4:	03100893          	li	a7,49
    __asm_syscall("r"(a7), "0"(a0))
    1ee8:	00000073          	ecall

int chdir(const char *path){
    return syscall(SYS_chdir, path);
}
    1eec:	2501                	sext.w	a0,a0
    1eee:	8082                	ret

0000000000001ef0 <mkdir>:

int mkdir(const char *path, mode_t mode){
    1ef0:	862e                	mv	a2,a1
    1ef2:	87aa                	mv	a5,a0
    register long a2 __asm__("a2") = c;
    1ef4:	1602                	slli	a2,a2,0x20
    register long a7 __asm__("a7") = n;
    1ef6:	02200893          	li	a7,34
    register long a0 __asm__("a0") = a;
    1efa:	f9c00513          	li	a0,-100
    register long a1 __asm__("a1") = b;
    1efe:	85be                	mv	a1,a5
    register long a2 __asm__("a2") = c;
    1f00:	9201                	srli	a2,a2,0x20
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1f02:	00000073          	ecall
    return syscall(SYS_mkdirat, AT_FDCWD, path, mode);
}
    1f06:	2501                	sext.w	a0,a0
    1f08:	8082                	ret

0000000000001f0a <getdents>:
    register long a7 __asm__("a7") = n;
    1f0a:	03d00893          	li	a7,61
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1f0e:	00000073          	ecall

int getdents(int fd, struct linux_dirent64 *dirp64, unsigned long len){
    //return syscall(SYS_getdents64, fd, dirp64, len);
    return syscall(SYS_getdents64, fd, dirp64, len);
}
    1f12:	2501                	sext.w	a0,a0
    1f14:	8082                	ret

0000000000001f16 <pipe>:
    register long a7 __asm__("a7") = n;
    1f16:	03b00893          	li	a7,59
    register long a1 __asm__("a1") = b;
    1f1a:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1f1c:	00000073          	ecall

int pipe(int fd[2]){
    return syscall(SYS_pipe2, fd, 0);
}
    1f20:	2501                	sext.w	a0,a0
    1f22:	8082                	ret

0000000000001f24 <dup>:
    register long a7 __asm__("a7") = n;
    1f24:	48dd                	li	a7,23
    __asm_syscall("r"(a7), "0"(a0))
    1f26:	00000073          	ecall

int dup(int fd){
    return syscall(SYS_dup, fd);
}
    1f2a:	2501                	sext.w	a0,a0
    1f2c:	8082                	ret

0000000000001f2e <dup2>:
    register long a7 __asm__("a7") = n;
    1f2e:	48e1                	li	a7,24
    register long a2 __asm__("a2") = c;
    1f30:	4601                	li	a2,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2))
    1f32:	00000073          	ecall

int dup2(int old, int new){
    return syscall(SYS_dup3, old, new, 0);
}
    1f36:	2501                	sext.w	a0,a0
    1f38:	8082                	ret

0000000000001f3a <mount>:
    register long a7 __asm__("a7") = n;
    1f3a:	02800893          	li	a7,40
    __asm_syscall("r"(a7), "0"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4))
    1f3e:	00000073          	ecall

int mount(const char *special, const char *dir, const char *fstype, unsigned long flags, const void *data)
{
        return syscall(SYS_mount, special, dir, fstype, flags, data);
}
    1f42:	2501                	sext.w	a0,a0
    1f44:	8082                	ret

0000000000001f46 <umount>:
    register long a7 __asm__("a7") = n;
    1f46:	02700893          	li	a7,39
    register long a1 __asm__("a1") = b;
    1f4a:	4581                	li	a1,0
    __asm_syscall("r"(a7), "0"(a0), "r"(a1))
    1f4c:	00000073          	ecall

int umount(const char *special)
{
        return syscall(SYS_umount2, special, 0);
}
    1f50:	2501                	sext.w	a0,a0
    1f52:	8082                	ret

0000000000001f54 <__clone>:

.global __clone
.type  __clone, %function
__clone:
	# Save func and arg to stack
	addi a1, a1, -16
    1f54:	15c1                	addi	a1,a1,-16
	sd a0, 0(a1)
    1f56:	e188                	sd	a0,0(a1)
	sd a3, 8(a1)
    1f58:	e594                	sd	a3,8(a1)

	# Call SYS_clone
	mv a0, a2
    1f5a:	8532                	mv	a0,a2
	mv a2, a4
    1f5c:	863a                	mv	a2,a4
	mv a3, a5
    1f5e:	86be                	mv	a3,a5
	mv a4, a6
    1f60:	8742                	mv	a4,a6
	li a7, 220 # SYS_clone
    1f62:	0dc00893          	li	a7,220
	ecall
    1f66:	00000073          	ecall

	beqz a0, 1f
    1f6a:	c111                	beqz	a0,1f6e <__clone+0x1a>
	# Parent
	ret
    1f6c:	8082                	ret

	# Child
1:      ld a1, 0(sp)
    1f6e:	6582                	ld	a1,0(sp)
	ld a0, 8(sp)
    1f70:	6522                	ld	a0,8(sp)
	jalr a1
    1f72:	9582                	jalr	a1

	# Exit
	li a7, 93 # SYS_exit
    1f74:	05d00893          	li	a7,93
	ecall
    1f78:	00000073          	ecall
