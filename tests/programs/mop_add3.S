.global _start
_start:
  /* ADD3A */
  li t0, 0x390a5a5fad56f578
  li t1, 0xdd8f0883fdd78883
  li t4, 0x7f6dbfa760006b9e
  add t0, t1, t0
  sltu t2, t0, t1
  add t3, t2, t4
  li s1, 0x169962e3ab2e7dfb
  li s0, 1
  bne t0, s1, fail
  li s1, 0xdd8f0883fdd78883
  li s0, 2
  bne t1, s1, fail
  li s1, 1
  li s0, 3
  bne t2, s1, fail
  li s1, 0x7f6dbfa760006b9f
  li s0, 4
  bne t3, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 5
  bne t4, s1, fail

  li t0, 1000
  li t1, 2000
  li t4, 4000
  add t0, t1, t0
  sltu t2, t0, t1
  add t3, t2, t4
  li s1, 3000
  li s0, 6
  bne t0, s1, fail
  li s1, 2000
  li s0, 7
  bne t1, s1, fail
  li s1, 0
  li s0, 8
  bne t2, s1, fail
  li s1, 4000
  li s0, 9
  bne t3, s1, fail
  li s1, 4000
  li s0, 10
  bne t4, s1, fail

  li t0, 0x390a5a5fad56f578
  li t1, 0xdd8f0883fdd78883
  li t4, 0x7f6dbfa760006b9e
  add t0, t1, t0
  sltu t2, t0, t1
  add t4, t2, t4
  li s1, 0x169962e3ab2e7dfb
  li s0, 11
  bne t0, s1, fail
  li s1, 0xdd8f0883fdd78883
  li s0, 12
  bne t1, s1, fail
  li s1, 1
  li s0, 13
  bne t2, s1, fail
  li s1, 0x7f6dbfa760006b9f
  li s0, 14
  bne t4, s1, fail

  li t0, 0x390a5a5fad56f578
  li t1, 0xdd8f0883fdd78883
  li t4, 0x7f6dbfa760006b9e
  add t0, t1, t0
  sltu t0, t0, t1
  add t3, t0, t4
  li s1, 1
  li s0, 15
  bne t0, s1, fail
  li s1, 0xdd8f0883fdd78883
  li s0, 16
  bne t1, s1, fail
  li s1, 0x7f6dbfa760006b9f
  li s0, 17
  bne t3, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 18
  bne t4, s1, fail

  li t0, 0x390a5a5fad56f578
  li t1, 0xdd8f0883fdd78883
  li t4, 0x7f6dbfa760006b9e
  add t0, t1, t0
  sltu t2, t0, t1
  add t0, t2, t4
  li s1, 0x7f6dbfa760006b9f
  li s0, 19
  bne t0, s1, fail
  li s1, 0xdd8f0883fdd78883
  li s0, 20
  bne t1, s1, fail
  li s1, 1
  li s0, 21
  bne t2, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 22
  bne t4, s1, fail

  /* ADD3B */
  li t1, 0xdd8f0883fdd78883
  li t2, 0x7f6dbfa760006b9e
  li t4, 0x390a5a5fad56f578
  add t0, t1, t2
  sltu t1, t0, t1
  add t3, t1, t4
  li s1, 0x5cfcc82b5dd7f421
  li s0, 23
  bne t0, s1, fail
  li s1, 1
  li s0, 24
  bne t1, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 25
  bne t2, s1, fail
  li s1, 0x390a5a5fad56f579
  li s0, 26
  bne t3, s1, fail
  li s1, 0x390a5a5fad56f578
  li s0, 27
  bne t4, s1, fail

  li t1, 3000
  li t2, 2000
  li t4, 4000
  add t0, t1, t2
  sltu t1, t0, t1
  add t3, t1, t4
  li s1, 5000
  li s0, 28
  bne t0, s1, fail
  li s1, 0
  li s0, 29
  bne t1, s1, fail
  li s1, 2000
  li s0, 30
  bne t2, s1, fail
  li s1, 4000
  li s0, 31
  bne t3, s1, fail
  li s1, 4000
  li s0, 32
  bne t4, s1, fail

  /* ADD3B r0 == r2 */
  li t1, 0xdd8f0883fdd78883
  li t0, 0x7f6dbfa760006b9e
  li t4, 0x390a5a5fad56f578
  add t0, t1, t0
  sltu t1, t0, t1
  add t3, t1, t4
  li s1, 0x5cfcc82b5dd7f421
  li s0, 33
  bne t0, s1, fail
  li s1, 1
  li s0, 34
  bne t1, s1, fail
  li s1, 0x390a5a5fad56f579
  li s0, 35
  bne t3, s1, fail
  li s1, 0x390a5a5fad56f578
  li s0, 36
  bne t4, s1, fail

  /* ADD3B r0 == r3 */
  li t1, 0xdd8f0883fdd78883
  li t2, 0x7f6dbfa760006b9e
  li t4, 0x390a5a5fad56f578
  add t0, t1, t2
  sltu t1, t0, t1
  add t0, t1, t4
  li s1, 0x390a5a5fad56f579
  li s0, 37
  bne t0, s1, fail
  li s1, 1
  li s0, 38
  bne t1, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 39
  bne t2, s1, fail
  li s1, 0x390a5a5fad56f578
  li s0, 40
  bne t4, s1, fail

  /* ADD3B r1 = r3 */
  li t1, 0xdd8f0883fdd78883
  li t2, 0x7f6dbfa760006b9e
  li t4, 0x390a5a5fad56f578
  add t0, t1, t2
  sltu t1, t0, t1
  add t1, t1, t4
  li s1, 0x5cfcc82b5dd7f421
  li s0, 41
  bne t0, s1, fail
  li s1, 0x390a5a5fad56f579
  li s0, 42
  bne t1, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 43
  bne t2, s1, fail
  li s1, 0x390a5a5fad56f578
  li s0, 44
  bne t4, s1, fail

  /* ADD3B r2 == r3 */
  li t1, 0xdd8f0883fdd78883
  li t2, 0x7f6dbfa760006b9e
  li t4, 0x390a5a5fad56f578
  add t0, t1, t2
  sltu t1, t0, t1
  add t2, t1, t4
  li s1, 0x5cfcc82b5dd7f421
  li s0, 45
  bne t0, s1, fail
  li s1, 1
  li s0, 46
  bne t1, s1, fail
  li s1, 0x390a5a5fad56f579
  li s0, 47
  bne t2, s1, fail
  li s1, 0x390a5a5fad56f578
  li s0, 48
  bne t4, s1, fail

  /* ADD3B r3 == r4 */
  li t1, 0xdd8f0883fdd78883
  li t2, 0x7f6dbfa760006b9e
  li t4, 0x390a5a5fad56f578
  add t0, t1, t2
  sltu t1, t0, t1
  add t4, t1, t4
  li s1, 0x5cfcc82b5dd7f421
  li s0, 49
  bne t0, s1, fail
  li s1, 1
  li s0, 50
  bne t1, s1, fail
  li s1, 0x7f6dbfa760006b9e
  li s0, 51
  bne t2, s1, fail
  li s1, 0x390a5a5fad56f579
  li s0, 52
  bne t4, s1, fail

  /* ADD3C */
  li t1, 0xad8f0883fdd78883
  li t2, 0x7f6db00760006b9e
  li t4, 0x390a5affff56f57f
  add t0, t1, t2
  sltu t3, t0, t1
  add t3, t3, t4
  li s1, 0x2cfcb88b5dd7f421
  li s0, 53
  bne t0, s1, fail
  li s1, 0xad8f0883fdd78883
  li s0, 54
  bne t1, s1, fail
  li s1, 0x7f6db00760006b9e
  li s0, 55
  bne t2, s1, fail
  li s1, 0x390a5affff56f580
  li s0, 56
  bne t3, s1, fail
  li s1, 0x390a5affff56f57f
  li s0, 57
  bne t4, s1, fail

  li t1, 1000
  li t2, 100
  li t4, 200
  add t0, t1, t2
  sltu t3, t0, t1
  add t3, t3, t4
  li s1, 1100
  li s0, 58
  bne t0, s1, fail
  li s1, 1000
  li s0, 59
  bne t1, s1, fail
  li s1, 100
  li s0, 60
  bne t2, s1, fail
  li s1, 200
  li s0, 61
  bne t3, s1, fail
  li s1, 200
  li s0, 62
  bne t4, s1, fail

  /* ADD3C r0 == r2 */
  li t1, 0xad8f0883fdd78883
  li t0, 0x7f6db00760006b9e
  li t4, 0x390a5affff56f57f
  add t0, t1, t0
  sltu t3, t0, t1
  add t3, t3, t4
  li s1, 0x2cfcb88b5dd7f421
  li s0, 63
  bne t0, s1, fail
  li s1, 0xad8f0883fdd78883
  li s0, 64
  bne t1, s1, fail
  li s1, 0x390a5affff56f580
  li s0, 65
  bne t3, s1, fail
  li s1, 0x390a5affff56f57f
  li s0, 66
  bne t4, s1, fail

  /* ADD3C r0 == r3 */
  li t1, 0xad8f0883fdd78883
  li t2, 0x7f6db00760006b9e
  li t4, 0x390a5affff56f57f
  add t0, t1, t2
  sltu t0, t0, t1
  add t0, t0, t4
  li s1, 0x390a5affff56f580
  li s0, 67
  bne t0, s1, fail
  li s1, 0xad8f0883fdd78883
  li s0, 68
  bne t1, s1, fail
  li s1, 0x7f6db00760006b9e
  li s0, 69
  bne t2, s1, fail
  li s1, 0x390a5affff56f57f
  li s0, 70
  bne t4, s1, fail

  /* ADD3C r1 == r3 */
  li t1, 0xad8f0883fdd78883
  li t2, 0x7f6db00760006b9e
  li t4, 0x390a5affff56f57f
  add t0, t1, t2
  sltu t1, t0, t1
  add t1, t1, t4
  li s1, 0x2cfcb88b5dd7f421
  li s0, 53
  bne t0, s1, fail
  li s1, 0x390a5affff56f580
  li s0, 54
  bne t1, s1, fail
  li s1, 0x7f6db00760006b9e
  li s0, 55
  bne t2, s1, fail
  li s1, 0x390a5affff56f57f
  li s0, 57
  bne t4, s1, fail

  /* ADD3C r2 == r3 */
  li t1, 0xad8f0883fdd78883
  li t3, 0x7f6db00760006b9e
  li t4, 0x390a5affff56f57f
  add t0, t1, t3
  sltu t3, t0, t1
  add t3, t3, t4
  li s1, 0x2cfcb88b5dd7f421
  li s0, 53
  bne t0, s1, fail
  li s1, 0xad8f0883fdd78883
  li s0, 54
  bne t1, s1, fail
  li s1, 0x390a5affff56f580
  li s0, 56
  bne t3, s1, fail
  li s1, 0x390a5affff56f57f
  li s0, 57
  bne t4, s1, fail

success:
  li a0, 0
  li a7, 93
  ecall
fail:
  li a0, 1
  li a7, 93
  ecall
