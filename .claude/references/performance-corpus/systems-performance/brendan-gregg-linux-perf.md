::: page
::: nav
Brendan\'s site:

[Start Here](/overview.html)\
[Homepage](/index.html)\
[Blog](/blog/index.html)\
[Sys Perf book](/systems-performance-2nd-edition-book.html)\
[BPF Perf book](/bpf-performance-tools-book.html)\
[Linux Perf](/linuxperf.html)\
[eBPF Tools](/ebpf.html)\
[perf Examples](/perf.html)\
[Perf Methods](/methodology.html)\
[USE Method](/usemethod.html)\
[TSA Method](/tsamethod.html)\
[Off-CPU Analysis](/offcpuanalysis.html)\
[Active Bench.](/activebenchmarking.html)\
[WSS Estimation](/wss.html)\
[Flame Graphs](/flamegraphs.html)\
[Flame Scope](/flamescope.html)\
[Heat Maps](/heatmaps.html)\
[Frequency Trails](/frequencytrails.html)\
[Colony Graphs](/colonygraphs.html)\
[DTrace Tools](/dtrace.html)\
[DTraceToolkit](/dtracetoolkit.html)\
[DtkshDemos](/dtkshdemos.html)\
[Guessing Game](/guessinggame.html)\
[Specials](/specials.html)\
[Books](/books.html)\
[Other Sites](/sites.html)\
:::

::: recent
[![](/Images/sysperf2nd_bookcover_360.jpg){width="180"}](/systems-performance-2nd-edition-book.html)\
*[Systems Performance 2nd Ed.](/systems-performance-2nd-edition-book.html)*

\
\

[![](/Images/bpfperftools_bookcover_360.jpg){width="180"}](/bpf-performance-tools-book.html)\
*[BPF Performance Tools book](/bpf-performance-tools-book.html)*

\
Recent posts:\

-   07 Feb 2026 »\
    [Why I joined OpenAI](/blog/2026-02-07/why-i-joined-openai.html)
-   05 Dec 2025 »\
    [Leaving Intel](/blog/2025-12-05/leaving-intel.html)
-   28 Nov 2025 »\
    [On \"AI Brendans\" or \"Virtual Brendans\"](/blog/2025-11-28/ai-virtual-brendans.html)
-   22 Nov 2025 »\
    [Intel is listening, don\'t waste your shot](/blog/2025-11-22/intel-is-listening.html)
-   17 Nov 2025 »\
    [Third Stage Engineering](/blog/2025-11-17/third-stage-engineering.html)
-   04 Aug 2025 »\
    [When to Hire a Computer Performance Engineering Team (2025) part 1 of 2](/blog/2025-08-04/when-to-hire-a-computer-performance-engineering-team-2025-part1.html)
-   22 May 2025 »\
    [3 Years of Extremely Remote Work](/blog/2025-05-22/3-years-of-extremely-remote-work.html)
-   01 May 2025 »\
    [Doom GPU Flame Graphs](/blog/2025-05-01/doom-gpu-flame-graphs.html)
-   29 Oct 2024 »\
    [AI Flame Graphs](/blog/2024-10-29/ai-flame-graphs.html)
-   22 Jul 2024 »\
    [No More Blue Fridays](/blog/2024-07-22/no-more-blue-fridays.html)
-   24 Mar 2024 »\
    [Linux Crisis Tools](/blog/2024-03-24/linux-crisis-tools.html)
-   17 Mar 2024 »\
    [The Return of the Frame Pointers](/blog/2024-03-17/the-return-of-the-frame-pointers.html)
-   10 Mar 2024 »\
    [eBPF Documentary](/blog/2024-03-10/ebpf-documentary.html)
-   28 Apr 2023 »\
    [eBPF Observability Tools Are Not Security Tools](/blog/2023-04-28/ebpf-security-issues.html)
-   01 Mar 2023 »\
    [USENIX SREcon APAC 2022: Computing Performance: What\'s on the Horizon](/blog/2023-03-01/computer-performance-future-2022.html)
-   17 Feb 2023 »\
    [USENIX SREcon APAC 2023: CFP](/blog/2023-02-17/srecon-apac-2023.html)
-   02 May 2022 »\
    [Brendan@Intel.com](/blog/2022-05-02/brendan-at-intel.html)
-   15 Apr 2022 »\
    [Netflix End of Series 1](/blog/2022-04-15/netflix-farewell-1.html)
-   09 Apr 2022 »\
    [TensorFlow Library Performance](/blog/2022-04-09/tensorflow-library-performance.html)
-   19 Mar 2022 »\
    [Why Don\'t You Use \...](/blog/2022-03-19/why-dont-you-use.html)

[Blog index](/blog/index.html)\
[About](/blog/about.html)\
[RSS](/blog/rss.xml)\
:::

::: content
# Linux Performance

::: {style="float:right;padding-left:30px;padding-right:10px;padding-bottom:9px"}
[![](Perf/linux_observability_tools.png){width="450" height="315" style="padding-bottom:3px"}](Perf/linux_observability_tools.png)\
static, benchmarking, tuning:\
[![](Perf/linux_static_tools_333.png){width="150" height="105" style="padding-bottom:3px"}](Perf/linux_static_tools.png) [![](Perf/linux_benchmarking_tools_333.png){width="167" height="105" style="padding-bottom:3px"}](Perf/linux_benchmarking_tools.png) [![](Perf/linux_tuning_tools_333.png){width="150" height="105" style="padding-bottom:3px"}](Perf/linux_tuning_tools.png)\
sar, [perf-tools](https://github.com/brendangregg/perf-tools#contents), [bcc/BPF](https://github.com/iovisor/bcc#tools):\
[![](Perf/linux_observability_sar_333.png){width="150" height="105" style="padding-bottom:3px"}](Perf/linux_observability_sar.png) [![](Perf/perf-tools_2016_333.png){width="150" height="105" style="padding-bottom:3px"}](Perf/perf-tools_2016.png) [![](Perf/bcc_tracing_tools_2016_333.png){width="150" height="105" style="padding-bottom:3px"}](Perf/bcc_tracing_tools.png)\
[bpftrace](https://github.com/iovisor/bpftrace), [BPF book](http://www.brendangregg.com/bpf-performance-tools-book.html):\
[![](Perf/linux_bpftrace_tools_300.png){width="150" height="85" style="padding-bottom:3px"}](Perf/linux_bpftrace_tools.png) [![](Perf/bpf_book_tools_300.png){width="150" height="105" style="padding-bottom:3px"}](Perf/bpf_book_tools.png)\
Images license: creative commons [Attribution-ShareAlike 4.0](http://creativecommons.org/licenses/by-sa/4.0/).\
:::

This page links to various Linux performance material I\'ve created, including the tools maps on the right. These use a large font size to suit slide decks. You can also print them out for your office wall. They show: [Linux observability tools](Perf/linux_observability_tools.png), [Linux static performance analysis tools](Perf/linux_static_tools.png), [Linux benchmarking tools](Perf/linux_benchmarking_tools.png), [Linux tuning tools](Perf/linux_tuning_tools.png), and [Linux sar](Perf/linux_observability_sar.png). Check the year on the image (bottom right) to see how recent it is.

There is also a hi-res diagram combining observability, static performance tuning, and perf-tools/bcc: [png](Perf/linux_perf_tools_full.png), [svg](Perf/linux_perf_tools_full.svg) (see [discussion](https://www.reddit.com/r/linux/comments/4x4smu/linux_performance_tools_full_version_draft/)), but it is not as complete as the other diagrams. For even more diagrams, see my slide decks below.

On this page: [Tools](#Tools), [Documentation](#Documentation), [Talks](#Talks), [Resources](#Resources).

[]{#Tools}

## Tools

-   [perf](perf.html): perf one-liners, examples, visualizations.
-   [eBPF tools](ebpf.html): BPF/bcc tracing tools and examples.
-   [perf-tools](https://github.com/brendangregg/perf-tools): Ftrace perf tools (github).
-   [bcc](https://github.com/iovisor/bcc#tools): BPF/bcc perf tools (github).
-   [bpftrace](https://github.com/iovisor/bpftrace#tools): BPF/bpftrace perf tools (github).
-   [Flame Graphs](flamegraphs.html): using [perf](FlameGraphs/cpuflamegraphs.html#perf) and other profilers.

[]{#Documentation}

## Documentation

-   [Linux Performance Analysis in 60,000 Milliseconds](http://techblog.netflix.com/2015/11/linux-performance-analysis-in-60s.html) shows the first ten commands to use in an investigation ([video](/blog/2015-12-03/linux-perf-60s-video.html), [PDF](/Articles/Netflix_Linux_Perf_Analysis_60s.pdf)). Written by myself and the performance engineering team at Netflix (2015).
-   My post [Performance Tuning Linux Instances on EC2](/blog/2015-03-03/performance-tuning-linux-instances-on-ec2.html) includes the tunables used at Netflix (2015).
-   A post on [Linux Load Averages: Solving the Mystery](/blog/2017-08-08/linux-load-averages.html), explaining what they are and why they include the uninterruptible sleep state (2017).
-   The post [The Return of the Frame Pointers](/blog/2024-03-17/the-return-of-the-frame-pointers.html), describing what they are, why they are coming back in major Linux distros, and other stack walking techniques (2024).
-   A [gdb Debugging Full Example (Tutorial)](/blog/2016-08-09/gdb-example-ncurses.html), including the use of some perf/debugging tools (2016).
-   The book [Systems Performance: Enterprise and the Cloud, 2nd Edition (2020)](systems-performance-2nd-edition-book.html) covers performance analysis methods and Linux tools, including perf, Ftrace, and eBPF.
-   The book [BPF Performance Tools: Linux System and Application Observability](bpf-performance-tools-book.html) tours over 100 eBPF performance analysis tools, while including short summaries of the traditional tools. In a way, this is volume 2, and Systems Performance 2nd Edition is volume 1.
-   Generating flame graphs on Linux using perf & eBPF:
-   Posts about eBPF, bcc, and bpftrace (2015-23):
-   My [lwn.net](http://lwn.net) article [Ftrace: The Hidden Light Switch](http://lwn.net/Articles/608497/) shows a use case for Linux ftrace (Aug, 2014).
-   Posts about ftrace-based perf-tools (2014-5):
-   Posts about perf-based perf-tools: [perf Hacktogram](/blog/2014-07-10/perf-hacktogram.html).
-   Posts about perf_events (2014-7):
-   A page on [Working Set Size Estimation](/wss.html) for Linux (2018+).
-   A post on [KPTI/KAISER Meltdown Initial Performance Regressions](/blog/2018-02-09/kpti-kaiser-meltdown-performance.html) (2018).
-   In [The PMCs of EC2: Measuring IPC](/blog/2017-05-04/the-pmcs-of-ec2.html) I showed the new Performance Monitoring Counter (PMC) support in the AWS EC2 cloud (2017).
-   [CPU Utilization is Wrong](/blog/2017-05-09/cpu-utilization-is-wrong.html): a post explaining the growing problem of memory stall cycles dominating the %CPU metric (2017).
-   A post about [Linux 4.7 Hist Triggers](http://www.brendangregg.com/blog/2016-06-08/linux-hist-triggers.html) (2016).
-   The post [The Speed of Time](/blog/2021-09-26/the-speed-of-time.html), a case study from 2014 of analyzing Linux clocksource performance in virtualized environments (2021).
-   The blog post [strace Wow Much Syscall](/blog/2014-05-11/strace-wow-much-syscall.html) discusses strace(1) for production use, and compares it to advanced tracing tools (2014).
-   [USE Method: Linux Performance Checklist](USEmethod/use-linux.html); also see the [USE Method](usemethod.html) page for the description of this methodology.
-   [Off-CPU Analysis Method](offcpuanalysis.html), where I demonstrate this methodology on Linux.

[]{#Talks}

## Talks

In rough order of recommended viewing or difficulty, intro to more advanced:

[]{#PerconaLive2016}

### 1. Linux Systems Performance (USENIX LISA 2019)

This is my summary of Linux systems performance in 40 minutes, covering six facets: observability, methodologies, benchmarking, profiling, tracing, and tuning. It\'s intended for everyone as a tour of fundamentals, and some companies have indicated they will use it for new hire training.

A video of the talk is on [usenix.org](https://www.usenix.org/conference/lisa19/presentation/gregg-linux) and [youtube](https://www.youtube.com/watch?v=fhBHvsi0Ql0&feature=emb_logo), here are the [slides](/Slides/LISA2019_Linux_Systems_Performance) or as a [PDF](/Slides/LISA2019_Linux_Systems_Performance.pdf).

For a lot more information on observability tools, profiling, and tracing, see the talks that follow.

[]{#PerconaLive2018}

### 2. Linux Performance 2018 (PerconaLive 2018)

This was a 20 minute keynote summary of recent changes and features in Linux performance in 2018.

A video of the talk is on [youtube](https://youtu.be/sV3XfrfjrPo?t=30m51s), and the slides are on [slideshare](https://www.slideshare.net/brendangregg/linux-performance-2018-perconalive-keynote-95516934) or as a

PDF.

[]{#Velocity2015}

### 3. Linux Performance Tools (Velocity 2015)

At Velocity 2015, I gave a 90 minute tutorial on Linux performance tools, summarizing performance observability, benchmarking, tuning, static performance tuning, and tracing tools. I also covered performance methodology, and included some live demos. This should be useful for everyone working on Linux systems. If you just saw my PerconaLive2016 talk, then some content should be familiar, but with many extras: I focus a lot more on the tools in this talk.

A video of the talk is on youtube ([playlist](https://www.youtube.com/watch?v=FJW8nGV4jxY&list=PLhhdIMVi0o5RNrf8E2dUijvGpqKLB9TCR); [part 1](https://www.youtube.com/watch?v=FJW8nGV4jxY), [part 2](https://www.youtube.com/watch?v=zrr2nUln9Kk)) and the slides are on [slideshare](http://www.slideshare.net/brendangregg/velocity-2015-linux-perf-tools) or as a [PDF](/Slides/Velocity2015_LinuxPerfTools.pdf).

This was similar to my [SCaLE11x](https://www.youtube.com/watch?v=0yyorhl6IjM) and [LinuxCon](/blog/2014-08-23/linux-perf-tools-linuxcon-na-2014.html) talks, however, with 90 minutes I was able to cover more tools and methodologies, making it the most complete tour of the topic I\'ve done. I also posted about it on the [Netflix Tech Blog](http://techblog.netflix.com/2015/08/netflix-at-velocity-2015-linux.html).

[]{#reInvent2017}

### 4. How Netflix Tunes EC2 Instances for Performance (AWS re:Invent, 2017)

Instead of performance observability, this talk is about tuning. I begin by providing Netflix background, covering instance types and features in the AWS EC2 cloud, and then talk about Linux kernel tunables and observability.

A video of the talk is on [youtube](https://www.youtube.com/watch?v=89fYOo1V2pA) and the slides are on [slideshare](https://www.slideshare.net/brendangregg/how-netflix-tunes-ec2-instances-for-performance):

[]{#DockerCon2017}

### 5. Container Performance Analysis (DockerCon, 2017)

At DockerCon 2017 in Austin, I gave a talk on Linux container performance analysis, showing how to find bottlenecks in the host vs the container, how to profiler container apps, and dig deeper into the kernel.

A video of the talk is on [youtube](https://www.youtube.com/watch?v=bK9A5ODIgac) and the slides are on [slideshare](https://www.slideshare.net/brendangregg/container-performance-analysis).

[]{#SCaLE14x}

### 6. Broken Linux Performance Tools (SCaLE14x, 2016)

At the Southern California Linux Expo ([SCaLE 14x](http://www.socallinuxexpo.org/scale/14x)), I gave a talk on Broken Linux Performance Tools. This was a follow-on to my earlier Linux Performance Tools talk originally at SCaLE11x (and more recently at [Velocity](#Velocity2015) as a tutorial). This broken tools talk was a tour of common problems with Linux system tools, metrics, statistics, visualizations, measurement overhead, and benchmarks. It also includes advice on how to cope (the green \"What You Can Do\" slides).

A video of the talk is on [youtube](https://www.youtube.com/watch?v=OPio8V-z03c) and the slides are on [slideshare](http://www.slideshare.net/brendangregg/broken-linux-performance-tools-2016) or as a [PDF](Slides/SCALE2016_Broken_Linux_Performance_Tools.pdf).

[]{#KernelRecipes2017}

### 7. Using Linux perf at Netflix (Kernel Recipes, 2017)

At [Kernel Recipes 2017](https://kernel-recipes.org/en/2017/talks/perf-in-netflix/) I gave an updated talk on Linux perf at Netflix, focusing on getting CPU profiling and flame graphs to work. This talk includes a crash course on perf_events, plus gotchas such as fixing stack traces and symbols when profiling Java, Node.js, VMs, and containers.

A video of the talk is on [youtube](https://www.youtube.com/watch?v=UVM3WX8Lq2k) and the slides are on [slideshare](https://www.slideshare.net/brendangregg/kernel-recipes-2017-using-linux-perf-at-netflix):

There\'s also an older version of this talk from 2015, which I\'ve [posted](/blog/2015-02-27/linux-profiling-at-netflix.html) about. To learn more about flame graphs, see my [flame graphs presentation](flamegraphs.html#Presentation).

### 8. Give me 15 minutes and I\'ll change your view of Linux tracing (LISA, 2016)

I gave this demo at USENIX/LISA 2016, showing ftrace, perf, and bcc/BPF. A video is on [youtube](https://www.youtube.com/watch?v=GsMs3n8CB6g) (sorry, the sound effects are a bit too loud):.

This was the first part of a longer talk on Linux 4.x Tracing Tools: Using BPF Superpowers. See the full [talk video](https://www.youtube.com/watch?v=UmOU3I36T2U) and [talk slides](http://www.slideshare.net/brendangregg/linux-4x-tracing-tools-using-bpf-superpowers).

### 9. Performance analysis superpowers with Linux eBPF (O\'Reilly Velocity, 2017)

This talk covers using enhanced BPF (aka eBPF) features added to the Linux 4.x series for performance analysis, observability, and debugging. The front-end used in this talk is bcc (BPF compiler collection), an open source project that provides BPF interfaces and a collection of tools.

A video of the talk is on [youtube](https://www.youtube.com/watch?v=bj3qdEDbCD4), and the slides are on [slideshare](https://www.slideshare.net/brendangregg/velocity-2017-performance-analysis-superpowers-with-linux-ebpf) or as a [PDF](/Slides/Velocity2017_BPF_superpowers.pdf).

[]{#LISA2014}

### 10. Linux Performance Analysis: New Tools and Old Secrets (ftrace) (LISA 2014)

At USENIX LISA 2014, I gave a talk on the new ftrace and perf_events tools I\'ve been developing: the [perf-tools](https://github.com/brendangregg/perf-tools) collection on github, which mostly uses ftrace: a tracer that has been built into the Linux kernel for many years, but few have discovered (practically a secret).

A video of the talk is on [youtube](https://www.youtube.com/watch?v=R4IKeMQhM0Y), and the slides are on [slideshare](http://www.slideshare.net/brendangregg/linux-performance-analysis-new-tools-and-old-secrets) or as a [PDF](/Slides/LISA2014_LinuxPerfAnalysisNewTools.pdf). In a [post](/blog/2015-03-17/usenix-lisa-2014-linux-ftrace-perf-tools.markdown) about this talk, I included some more screenshots of these tools in action.

### 11. Performance Checklists for SREs (SREcon, 2016)

At [SREcon 2016 Santa Clara](https://www.usenix.org/conference/srecon16/program), I gave the closing talk on performance checklists for SREs (Site Reliability Engineers). The later half of this talk included Linux checklists for incident performance response. These may be useful whether you\'re analyzing Linux performance in a hurry or not.

A video of the talk is on [youtube](https://www.youtube.com/watch?v=zxCWXNigDpA) and [usenix](https://www.usenix.org/conference/srecon16/program/presentation/gregg), and the slides are on [slideshare](http://www.slideshare.net/brendangregg/srecon-2016-performance-checklists-for-sres) and as a [PDF](Slides/SREcon_2016_perf_checklists.pdf). I included the checklists in a [blog post](/blog/2016-05-04/srecon2016-perf-checklists-for-sres.html).

[]{#Resources}

## Resources

Other resources (not by me) I\'d recommend for the topic of Linux performance:

-   [RHEL Performance Guide](https://github.com/myllynen/rhel-performance-guide) includes many CLI tools and tunables.
-   [Performance analysis & tuning of Red Hat Enterprise Linux - 2015 Red Hat Summit](https://www.youtube.com/watch?v=ckarvGJE8Qc) (video 2hrs): this is a great and in-depth tour of Linux performance tuning that should be largely applicable to all Linux distros.
-   [Linux Instrumentation](http://www.slideshare.net/DarkStarSword/instrumentation): slides from a great talk in June 2010 by Ian Munsie, which summarizes the different Linux tracers very well. If you\'re trying to understand all the tracers and frameworks, this is worth studying (keeping in mind it\'s from 2010).
-   [Julia Evans blog](http://jvns.ca/) has many posts about many topics, including performance tools.
-   [Davidlohr Bueso\'s](http://blog.stgolabs.net/search/label/linux) Linux performance posts.
:::

::: navmobile

------------------------------------------------------------------------

Site Navigation

\

------------------------------------------------------------------------
:::

::: recentmobile
[![](/Images/sysperf2nd_bookcover_360.jpg){width="180"}](/systems-performance-2nd-edition-book.html)\
*[Systems Performance 2nd Ed.](/systems-performance-2nd-edition-book.html)*

\
\

[![](/Images/bpfperftools_bookcover_360.jpg){width="180"}](/bpf-performance-tools-book.html)\
*[BPF Performance Tools book](/bpf-performance-tools-book.html)*

\
Recent posts:\

-   07 Feb 2026 »\
    [Why I joined OpenAI](/blog/2026-02-07/why-i-joined-openai.html)
-   05 Dec 2025 »\
    [Leaving Intel](/blog/2025-12-05/leaving-intel.html)
-   28 Nov 2025 »\
    [On \"AI Brendans\" or \"Virtual Brendans\"](/blog/2025-11-28/ai-virtual-brendans.html)
-   22 Nov 2025 »\
    [Intel is listening, don\'t waste your shot](/blog/2025-11-22/intel-is-listening.html)
-   17 Nov 2025 »\
    [Third Stage Engineering](/blog/2025-11-17/third-stage-engineering.html)
-   04 Aug 2025 »\
    [When to Hire a Computer Performance Engineering Team (2025) part 1 of 2](/blog/2025-08-04/when-to-hire-a-computer-performance-engineering-team-2025-part1.html)
-   22 May 2025 »\
    [3 Years of Extremely Remote Work](/blog/2025-05-22/3-years-of-extremely-remote-work.html)
-   01 May 2025 »\
    [Doom GPU Flame Graphs](/blog/2025-05-01/doom-gpu-flame-graphs.html)
-   29 Oct 2024 »\
    [AI Flame Graphs](/blog/2024-10-29/ai-flame-graphs.html)
-   22 Jul 2024 »\
    [No More Blue Fridays](/blog/2024-07-22/no-more-blue-fridays.html)
-   24 Mar 2024 »\
    [Linux Crisis Tools](/blog/2024-03-24/linux-crisis-tools.html)
-   17 Mar 2024 »\
    [The Return of the Frame Pointers](/blog/2024-03-17/the-return-of-the-frame-pointers.html)
-   10 Mar 2024 »\
    [eBPF Documentary](/blog/2024-03-10/ebpf-documentary.html)
-   28 Apr 2023 »\
    [eBPF Observability Tools Are Not Security Tools](/blog/2023-04-28/ebpf-security-issues.html)
-   01 Mar 2023 »\
    [USENIX SREcon APAC 2022: Computing Performance: What\'s on the Horizon](/blog/2023-03-01/computer-performance-future-2022.html)
-   17 Feb 2023 »\
    [USENIX SREcon APAC 2023: CFP](/blog/2023-02-17/srecon-apac-2023.html)
-   02 May 2022 »\
    [Brendan@Intel.com](/blog/2022-05-02/brendan-at-intel.html)
-   15 Apr 2022 »\
    [Netflix End of Series 1](/blog/2022-04-15/netflix-farewell-1.html)
-   09 Apr 2022 »\
    [TensorFlow Library Performance](/blog/2022-04-09/tensorflow-library-performance.html)
-   19 Mar 2022 »\
    [Why Don\'t You Use \...](/blog/2022-03-19/why-dont-you-use.html)

[Blog index](/blog/index.html)\
[About](/blog/about.html)\
[RSS](/blog/rss.xml)\
:::
:::

------------------------------------------------------------------------

Last updated: 17-Mar-2024\
Copyright 2024 Brendan Gregg, all rights reserved\
