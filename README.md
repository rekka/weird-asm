Example of `--emit=asm` passed to `rustc` changing the produced binary.

```
$ rustc --version
rustc 1.26.0 (a77568041 2018-05-07)
```

But the same behavior occurs on the latest nightly (`rustc
1.28.0-nightly (990d8aa74 2018-05-25)`).

I used the following code (script `dump.sh`) on Ubuntu 17.10, Haswell Core i7 CPU.

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
objdump -d target/release/weird-asm > objdump.txt

RUSTFLAGS="-C target-cpu=native --emit=asm" cargo build --release
objdump -d target/release/weird-asm > objdump-emit-asm.txt
```

The same happens if `cargo rustc ... -- ...` is used directly.

I observe a similar behavior on a MacBook with a Broadwell Core i5 CPU.

In particular, the binary without `--emit=asm` seems much better optimized
(vectorized SIMD is used).

Example:

## No `--emit=asm` (`objdump.txt`)

```
7ab0:	c4 c1 6d 59 74 f2 d8 	vmulpd -0x28(%r10,%rsi,8),%ymm2,%ymm6
7ab7:	c5 fd 10 7c f1 e0    	vmovupd -0x20(%rcx,%rsi,8),%ymm7
7abd:	c4 c1 45 58 7c f2 d0 	vaddpd -0x30(%r10,%rsi,8),%ymm7,%ymm7
7ac4:	c4 c1 45 58 7c f2 e0 	vaddpd -0x20(%r10,%rsi,8),%ymm7,%ymm7
7acb:	c5 c5 58 7c f0 e0    	vaddpd -0x20(%rax,%rsi,8),%ymm7,%ymm7
7ad1:	c4 c1 45 58 7c f0 08 	vaddpd 0x8(%r8,%rsi,8),%ymm7,%ymm7
7ad8:	c5 c5 59 fb          	vmulpd %ymm3,%ymm7,%ymm7
7adc:	c4 c1 45 59 7c f1 e0 	vmulpd -0x20(%r9,%rsi,8),%ymm7,%ymm7
7ae3:	c5 cd 58 f7          	vaddpd %ymm7,%ymm6,%ymm6
7ae7:	c4 c1 7d 11 74 f4 e0 	vmovupd %ymm6,-0x20(%r12,%rsi,8)
7aee:	c4 c1 6d 59 74 f2 f8 	vmulpd -0x8(%r10,%rsi,8),%ymm2,%ymm6
7af5:	c5 fd 10 3c f1       	vmovupd (%rcx,%rsi,8),%ymm7
7afa:	c4 c1 45 58 7c f2 f0 	vaddpd -0x10(%r10,%rsi,8),%ymm7,%ymm7
7b01:	c4 c1 45 58 3c f2    	vaddpd (%r10,%rsi,8),%ymm7,%ymm7
7b07:	c5 c5 58 3c f0       	vaddpd (%rax,%rsi,8),%ymm7,%ymm7
7b0c:	c4 c1 45 58 7c f0 28 	vaddpd 0x28(%r8,%rsi,8),%ymm7,%ymm7
7b13:	c5 c5 59 fb          	vmulpd %ymm3,%ymm7,%ymm7
7b17:	c4 c1 45 59 3c f1    	vmulpd (%r9,%rsi,8),%ymm7,%ymm7
7b1d:	c5 cd 58 f7          	vaddpd %ymm7,%ymm6,%ymm6
7b21:	c4 c1 7d 11 34 f4    	vmovupd %ymm6,(%r12,%rsi,8)
7b27:	48 83 c6 08          	add    $0x8,%rsi
7b2b:	48 83 c5 02          	add    $0x2,%rbp
7b2f:	0f 85 7b ff ff ff    	jne    7ab0 <_ZN9weird_asm21precomp_damped_jacobi17h4c145fd63e2bf0d0E+0x390>
```

## With `--emit=asm` (`objdump-emit-asm.txt`)

```
79b0:	c5 fb 10 6c c5 f8    	vmovsd -0x8(%rbp,%rax,8),%xmm5
79b6:	48 83 c0 01          	add    $0x1,%rax
79ba:	c5 f3 59 f4          	vmulsd %xmm4,%xmm1,%xmm6
79be:	c4 c1 53 58 2c c3    	vaddsd (%r11,%rax,8),%xmm5,%xmm5
79c4:	c5 fb 10 64 c5 00    	vmovsd 0x0(%rbp,%rax,8),%xmm4
79ca:	c5 d3 58 ec          	vaddsd %xmm4,%xmm5,%xmm5
79ce:	c4 c1 53 58 2c c0    	vaddsd (%r8,%rax,8),%xmm5,%xmm5
79d4:	c4 c1 53 58 2c c6    	vaddsd (%r14,%rax,8),%xmm5,%xmm5
79da:	c5 d3 59 e8          	vmulsd %xmm0,%xmm5,%xmm5
79de:	c4 c1 53 59 2c c2    	vmulsd (%r10,%rax,8),%xmm5,%xmm5
79e4:	c5 cb 58 ed          	vaddsd %xmm5,%xmm6,%xmm5
79e8:	c4 c1 7b 11 6c c5 00 	vmovsd %xmm5,0x0(%r13,%rax,8)
79ef:	49 39 c1             	cmp    %rax,%r9
79f2:	75 bc                	jne    79b0 <_ZN9weird_asm21precomp_damped_jacobi17h4c145fd63e2bf0d0E+0x290>
```
