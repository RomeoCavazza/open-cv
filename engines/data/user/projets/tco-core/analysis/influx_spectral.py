#!/usr/bin/env python3
import argparse
import os
import sys
from datetime import datetime

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy import signal
from influxdb import InfluxDBClient


def mkdir_p(path: str) -> None:
    if path:
        os.makedirs(path, exist_ok=True)


def influx_fetch(host: str, port: int, db: str, measurement: str, hours: float) -> pd.DataFrame:
    client = InfluxDBClient(host=host, port=port, database=db)
    secs = int(round(hours * 3600))
    q = f'SELECT * FROM "{measurement}" WHERE time > now() - {secs}s'
    result = client.query(q)
    pts = list(result.get_points())
    if not pts:
        return pd.DataFrame()
    df = pd.DataFrame(pts)
    df["time"] = pd.to_datetime(df["time"], utc=True)
    df = df.set_index("time").sort_index()
    return df


def preprocess(df: pd.DataFrame, fields: list[str], fs: float) -> pd.DataFrame:
    missing = [f for f in fields if f not in df.columns]
    if missing:
        raise KeyError(f"Champs absents: {missing}. Champs disponibles: {list(df.columns)}")

    x = df[fields].apply(pd.to_numeric, errors="coerce").replace([np.inf, -np.inf], np.nan).dropna()
    if x.empty:
        raise ValueError("Données vides après nettoyage (NaN/Inf).")

    dt_ms = int(round(1000.0 / fs))
    x = x.resample(f"{dt_ms}ms").mean().interpolate(limit_direction="both")
    return x


def zscore(df: pd.DataFrame) -> pd.DataFrame:
    return (df - df.mean()) / df.std(ddof=0)


def savefig(fig, outdir: str, name: str) -> None:
    if outdir:
        fig.savefig(os.path.join(outdir, name), dpi=150, bbox_inches="tight")
        plt.close(fig)
    else:
        plt.show()


def plot_timeseries(df: pd.DataFrame, outdir: str) -> None:
    fig, ax = plt.subplots(figsize=(15, 5))
    d = zscore(df)
    for col in d.columns:
        ax.plot(d.index, d[col].values, label=col, alpha=0.85)
    ax.set_title("Time series (z-score)")
    ax.set_ylabel("σ")
    ax.grid(True)
    ax.legend()
    savefig(fig, outdir, "01_timeseries.png")


def plot_psd(df: pd.DataFrame, fs: float, nperseg: int, outdir: str) -> None:
    fig, ax = plt.subplots(figsize=(15, 5))
    for col in df.columns:
        x = df[col].to_numpy(dtype=float)
        x = x - np.mean(x)
        f, pxx = signal.welch(x, fs=fs, nperseg=min(nperseg, len(x)))
        ax.semilogy(f, pxx, label=col)
    ax.set_title(f"PSD (Welch, nperseg={nperseg})")
    ax.set_xlabel("Frequency [Hz]")
    ax.set_ylabel("PSD")
    ax.grid(True, which="both", alpha=0.5)
    ax.legend()
    savefig(fig, outdir, "02_psd_welch.png")


def plot_spectrogram(df: pd.DataFrame, fs: float, nperseg: int, outdir: str) -> None:
    for col in df.columns:
        x = df[col].to_numpy(dtype=float)
        x = x - np.mean(x)
        f, t, Sxx = signal.spectrogram(
            x, fs=fs, nperseg=min(nperseg, len(x)), scaling="density", mode="psd"
        )
        fig, ax = plt.subplots(figsize=(15, 5))
        Z = 10.0 * np.log10(Sxx + 1e-20)
        pcm = ax.pcolormesh(t, f, Z, shading="auto")
        ax.set_title(f"Spectrogram ({col})")
        ax.set_xlabel("Time [s]")
        ax.set_ylabel("Frequency [Hz]")
        fig.colorbar(pcm, ax=ax, label="Power [dB]")
        savefig(fig, outdir, f"03_spectrogram_{col}.png")


def plot_coherence(df: pd.DataFrame, a: str, b: str, fs: float, nperseg: int, outdir: str) -> None:
    x = df[a].to_numpy(dtype=float)
    y = df[b].to_numpy(dtype=float)
    x = x - np.mean(x)
    y = y - np.mean(y)
    f, cxy = signal.coherence(x, y, fs=fs, nperseg=min(nperseg, len(x)))
    fig, ax = plt.subplots(figsize=(15, 5))
    ax.plot(f, cxy)
    ax.set_title(f"Coherence ({a} vs {b})")
    ax.set_xlabel("Frequency [Hz]")
    ax.set_ylabel("Coherence")
    ax.set_ylim(0, 1.05)
    ax.grid(True, alpha=0.5)
    savefig(fig, outdir, f"04_coherence_{a}_{b}.png")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="192.168.1.191")
    ap.add_argument("--port", type=int, default=8086)
    ap.add_argument("--db", default="tpu_db")
    ap.add_argument("--measurement", default="tpu_sensors")
    ap.add_argument("--fields", default="az,temp")
    ap.add_argument("--hours", type=float, default=9.0)
    ap.add_argument("--fs", type=float, default=10.0)
    ap.add_argument("--nperseg", type=int, default=1024)
    ap.add_argument("--out", default="analysis_output")
    args = ap.parse_args()

    fields = [f.strip() for f in args.fields.split(",") if f.strip()]
    if not fields:
        print("Aucun champ fourni.", file=sys.stderr)
        return 2
    if args.fs <= 0:
        print("fs doit être > 0.", file=sys.stderr)
        return 2

    outdir = args.out.strip()
    if outdir == "":
        outdir = None
    else:
        mkdir_p(outdir)

    raw = influx_fetch(args.host, args.port, args.db, args.measurement, args.hours)
    if raw.empty:
        print("Aucune donnée retournée par InfluxDB.", file=sys.stderr)
        return 1

    df = preprocess(raw, fields, args.fs)

    if outdir:
        stamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        df.to_csv(os.path.join(outdir, f"data_resampled_{stamp}.csv"))

    plot_timeseries(df, outdir)
    plot_psd(df, args.fs, args.nperseg, outdir)
    plot_spectrogram(df, args.fs, args.nperseg, outdir)

    if len(fields) == 2:
        plot_coherence(df, fields[0], fields[1], args.fs, args.nperseg, outdir)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
