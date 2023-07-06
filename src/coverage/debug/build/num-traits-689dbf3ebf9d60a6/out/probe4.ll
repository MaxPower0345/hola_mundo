; ModuleID = 'probe4.e653e01e-cgu.0'
source_filename = "probe4.e653e01e-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_82B731871106174Eu = comdat any

$__llvm_profile_filename = comdat any

@__covrec_82B731871106174Eu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -9027692471864912050, i32 9, i64 2731335887197271148, i64 -2272513194550435862, [9 x i8] c"\01\01\00\01\01\01\01\001" }>, section ".lcovfun$M", comdat, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [96 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 96, i32 0, i32 5 }, [96 x i8] c"\02]\00UC:\\Users\\fabri\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.15\06<anon>" }, section ".lcovmap$M", align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCsbD8h8aN09vC_6probe45probe = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCsbD8h8aN09vC_6probe45probe = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 -9027692471864912050, i64 2731335887197271148, i64 sub (i64 ptrtoint (ptr @__profc__RNvCsbD8h8aN09vC_6probe45probe to i64), i64 ptrtoint (ptr @__profd__RNvCsbD8h8aN09vC_6probe45probe to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCsbD8h8aN09vC_6probe45probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCsbD8h8aN09vC_6probe45probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__covrec_82B731871106174Eu, ptr @__llvm_coverage_mapping, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = hidden constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; probe4::probe
; Function Attrs: uwtable
define void @_RNvCsbD8h8aN09vC_6probe45probe() unnamed_addr #0 !dbg !5 {
start:
  %0 = alloca i32, align 4
  %self.dbg.spill2.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  %pgocount = load i64, ptr @__profc__RNvCsbD8h8aN09vC_6probe45probe, align 8, !dbg !11
  %1 = add i64 %pgocount, 1, !dbg !11
  store i64 %1, ptr @__profc__RNvCsbD8h8aN09vC_6probe45probe, align 8, !dbg !11
  store i32 1, ptr %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill.i, metadata !12, metadata !DIExpression()), !dbg !24
  store i32 -2, ptr %self.dbg.spill2.i, align 4, !dbg !26
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2.i, metadata !27, metadata !DIExpression()), !dbg !30
  store i32 1, ptr %0, align 4, !dbg !32
  %2 = load i32, ptr %0, align 4, !dbg !32, !noundef !10
  ret void, !dbg !33
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.cttz.i32(i32, i1 immarg) #1

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #2

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #3 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { nounwind }
attributes #3 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.70.0 (90c541806 2023-05-31))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe4\\@\\probe4.e653e01e-cgu.0", directory: "C:\\Users\\fabri\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.15")
!5 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCsbD8h8aN09vC_6probe45probe", scope: !7, file: !6, line: 1, type: !8, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !10)
!6 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA1, checksum: "e59aad6526d2abf9a5e6a13068d52e15c66f45e9")
!7 = !DINamespace(name: "probe4", scope: null)
!8 = !DISubroutineType(types: !9)
!9 = !{null}
!10 = !{}
!11 = !DILocation(line: 1, scope: !5)
!12 = !DILocalVariable(name: "self", arg: 1, scope: !13, file: !14, line: 210, type: !20)
!13 = distinct !DISubprogram(name: "trailing_ones", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$13trailing_ones17hb38f179178e2d8a4E", scope: !15, file: !14, line: 210, type: !18, scopeLine: 210, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !23)
!14 = !DIFile(filename: "/rustc/90c541806f23a127002de5b4038be731ba1458ca\\library\\core\\src\\num\\uint_macros.rs", directory: "", checksumkind: CSK_SHA1, checksum: "a56e26d32f8cd34c7311818fe3801f3e68ea12d0")
!15 = !DINamespace(name: "impl$8", scope: !16)
!16 = !DINamespace(name: "num", scope: !17)
!17 = !DINamespace(name: "core", scope: null)
!18 = !DISubroutineType(types: !19)
!19 = !{!20, !20}
!20 = !DIDerivedType(tag: DW_TAG_typedef, name: "u32", file: !21, baseType: !22)
!21 = !DIFile(filename: "<unknown>", directory: "")
!22 = !DIBasicType(name: "unsigned __int32", size: 32, encoding: DW_ATE_unsigned)
!23 = !{!12}
!24 = !DILocation(line: 210, scope: !13, inlinedAt: !25)
!25 = distinct !DILocation(line: 1, scope: !5)
!26 = !DILocation(line: 211, scope: !13, inlinedAt: !25)
!27 = !DILocalVariable(name: "self", arg: 1, scope: !28, file: !14, line: 169, type: !20)
!28 = distinct !DISubprogram(name: "trailing_zeros", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$14trailing_zeros17he721984cf38d96b9E", scope: !15, file: !14, line: 169, type: !18, scopeLine: 169, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !29)
!29 = !{!27}
!30 = !DILocation(line: 169, scope: !28, inlinedAt: !31)
!31 = distinct !DILocation(line: 211, scope: !13, inlinedAt: !25)
!32 = !DILocation(line: 170, scope: !28, inlinedAt: !31)
!33 = !DILocation(line: 1, scope: !34)
!34 = !DILexicalBlockFile(scope: !5, file: !6, discriminator: 0)
