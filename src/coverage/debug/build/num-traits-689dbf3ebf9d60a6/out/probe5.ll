; ModuleID = 'probe5.2f5b39cf-cgu.0'
source_filename = "probe5.2f5b39cf-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_98E69FB6DB6AD7Cu = comdat any

$__llvm_profile_filename = comdat any

@alloc_4b4a986058e6905f521e49e6f6f5449f = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/90c541806f23a127002de5b4038be731ba1458ca\\library\\core\\src\\ops\\arith.rs" }>, align 1
@alloc_0177345fd4df401ab8b221ba99f44cf3 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4b4a986058e6905f521e49e6f6f5449f, [16 x i8] c"M\00\00\00\00\00\00\00\08\03\00\00\01\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc_2e38410fced2c310c68bdf2d45d0c3bd = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\02\00\00\00" }>, align 4
@__covrec_98E69FB6DB6AD7Cu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 688604321632660860, i32 9, i64 8591518952133608102, i64 -2272513194550435862, [9 x i8] c"\01\01\00\01\01\01\01\008" }>, section ".lcovfun$M", comdat, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [96 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 96, i32 0, i32 5 }, [96 x i8] c"\02]\00UC:\\Users\\fabri\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.15\06<anon>" }, section ".lcovmap$M", align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCsk6ZbQKAN8df_6probe55probe = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCsk6ZbQKAN8df_6probe55probe = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 688604321632660860, i64 8591518952133608102, i64 sub (i64 ptrtoint (ptr @__profc__RNvCsk6ZbQKAN8df_6probe55probe to i64), i64 ptrtoint (ptr @__profd__RNvCsk6ZbQKAN8df_6probe55probe to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCsk6ZbQKAN8df_6probe55probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCsk6ZbQKAN8df_6probe55probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__covrec_98E69FB6DB6AD7Cu, ptr @__llvm_coverage_mapping, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = hidden constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint uwtable
define internal void @_RNvXs57_NtNtCsgdXEgQvlTtX_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsk6ZbQKAN8df_6probe5(ptr align 4 %self, ptr align 4 %other) unnamed_addr #0 !dbg !5 {
start:
  %other.dbg.spill2 = alloca i32, align 4
  %other.dbg.spill = alloca ptr, align 8
  %self.dbg.spill = alloca ptr, align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !19, metadata !DIExpression()), !dbg !22
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !23, metadata !DIExpression()), !dbg !31
  store ptr %other, ptr %other.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill, metadata !20, metadata !DIExpression()), !dbg !22
  %other1 = load i32, ptr %other, align 4, !dbg !32, !noundef !21
  store i32 %other1, ptr %other.dbg.spill2, align 4, !dbg !32
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill2, metadata !30, metadata !DIExpression()), !dbg !31
  %0 = load i32, ptr %self, align 4, !dbg !31, !noundef !21
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %other1), !dbg !31
  %_4.0 = extractvalue { i32, i1 } %1, 0, !dbg !31
  %_4.1 = extractvalue { i32, i1 } %1, 1, !dbg !31
  %2 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false), !dbg !31
  br i1 %2, label %panic, label %bb1, !dbg !31

bb1:                                              ; preds = %start
  store i32 %_4.0, ptr %self, align 4, !dbg !31
  ret void, !dbg !33

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hb92e8c073af93b07E(ptr align 1 @str.0, i64 28, ptr align 8 @alloc_0177345fd4df401ab8b221ba99f44cf3) #7, !dbg !31
  unreachable, !dbg !31
}

; probe5::probe
; Function Attrs: uwtable
define void @_RNvCsk6ZbQKAN8df_6probe55probe() unnamed_addr #1 !dbg !34 {
start:
  %x = alloca i32, align 4
  call void @llvm.dbg.declare(metadata ptr %x, metadata !40, metadata !DIExpression()), !dbg !42
  %pgocount = load i64, ptr @__profc__RNvCsk6ZbQKAN8df_6probe55probe, align 8, !dbg !42
  %0 = add i64 %pgocount, 1, !dbg !42
  store i64 %0, ptr @__profc__RNvCsk6ZbQKAN8df_6probe55probe, align 8, !dbg !42
  store i32 1, ptr %x, align 4, !dbg !43
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @_RNvXs57_NtNtCsgdXEgQvlTtX_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsk6ZbQKAN8df_6probe5(ptr align 4 %x, ptr align 4 @alloc_2e38410fced2c310c68bdf2d45d0c3bd), !dbg !42
  ret void, !dbg !44
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hb92e8c073af93b07E(ptr align 1, i64, ptr align 8) unnamed_addr #4

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #5

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #6 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #4 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #5 = { nounwind }
attributes #6 = { noinline }
attributes #7 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.70.0 (90c541806 2023-05-31))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe5\\@\\probe5.2f5b39cf-cgu.0", directory: "C:\\Users\\fabri\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\num-traits-0.2.15")
!5 = distinct !DISubprogram(name: "add_assign", linkageName: "_RNvXs57_NtNtCsgdXEgQvlTtX_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsk6ZbQKAN8df_6probe5", scope: !7, file: !6, line: 126, type: !11, scopeLine: 126, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !21, retainedNodes: !18)
!6 = !DIFile(filename: "/rustc/90c541806f23a127002de5b4038be731ba1458ca\\library\\core\\src\\internal_macros.rs", directory: "", checksumkind: CSK_SHA1, checksum: "82535feef85940682309f24327f159e8b2daa71a")
!7 = !DINamespace(name: "impl$319", scope: !8)
!8 = !DINamespace(name: "arith", scope: !9)
!9 = !DINamespace(name: "ops", scope: !10)
!10 = !DINamespace(name: "core", scope: null)
!11 = !DISubroutineType(types: !12)
!12 = !{null, !13, !17}
!13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "ref_mut$<i32>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!14 = !DIDerivedType(tag: DW_TAG_typedef, name: "i32", file: !15, baseType: !16)
!15 = !DIFile(filename: "<unknown>", directory: "")
!16 = !DIBasicType(name: "__int32", size: 32, encoding: DW_ATE_signed)
!17 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "ref$<i32>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!18 = !{!19, !20}
!19 = !DILocalVariable(name: "self", arg: 1, scope: !5, file: !6, line: 126, type: !13)
!20 = !DILocalVariable(name: "other", arg: 2, scope: !5, file: !6, line: 126, type: !17)
!21 = !{}
!22 = !DILocation(line: 126, scope: !5)
!23 = !DILocalVariable(name: "self", arg: 1, scope: !24, file: !25, line: 769, type: !13)
!24 = distinct !DISubprogram(name: "add_assign", linkageName: "_RNvXs4T_NtNtCsgdXEgQvlTtX_4core3ops5arithlNtB6_9AddAssign10add_assignCsk6ZbQKAN8df_6probe5", scope: !26, file: !25, line: 769, type: !27, scopeLine: 769, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !21, retainedNodes: !29)
!25 = !DIFile(filename: "/rustc/90c541806f23a127002de5b4038be731ba1458ca\\library\\core\\src\\ops\\arith.rs", directory: "", checksumkind: CSK_SHA1, checksum: "3c78c1639a414a6269c9787288eaa9bca4318c2b")
!26 = !DINamespace(name: "impl$305", scope: !8)
!27 = !DISubroutineType(types: !28)
!28 = !{null, !13, !14}
!29 = !{!23, !30}
!30 = !DILocalVariable(name: "other", arg: 2, scope: !24, file: !25, line: 769, type: !14)
!31 = !DILocation(line: 769, scope: !24, inlinedAt: !32)
!32 = !DILocation(line: 127, scope: !5)
!33 = !DILocation(line: 128, scope: !5)
!34 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCsk6ZbQKAN8df_6probe55probe", scope: !36, file: !35, line: 1, type: !37, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !21, retainedNodes: !39)
!35 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA1, checksum: "c32193a83b8a6643d57348f9ba2e78a51840976b")
!36 = !DINamespace(name: "probe5", scope: null)
!37 = !DISubroutineType(types: !38)
!38 = !{null}
!39 = !{!40}
!40 = !DILocalVariable(name: "x", scope: !41, file: !35, line: 1, type: !14, align: 4)
!41 = distinct !DILexicalBlock(scope: !34, file: !35, line: 1)
!42 = !DILocation(line: 1, scope: !41)
!43 = !DILocation(line: 1, scope: !34)
!44 = !DILocation(line: 1, scope: !45)
!45 = !DILexicalBlockFile(scope: !34, file: !35, discriminator: 0)
