# vst-sine
Test different implementations to generate a sine wave.

# 32 Bit and 64 Bit Floats

In these implementations, time tracked in seconds. All calculations are completely in `f32`or `f64`. The 32 bit implementation quickly accumulates floating point errors, whereas the 64 bit implementation works for hours (I've literally tested that ;-)).

## 32-bit spectrum and waveform:

![Spectrum](https://raw.githubusercontent.com/bitzl/vst-sine/master/images/32bit-spectrum.png)	

![Waveform](https://raw.githubusercontent.com/bitzl/vst-sine/master/images/32bit-waveform.png)

Note that the breaks in the waveform move because of increasing floating point errors


## 64-bit spectrum and waveform:

![Spectrum](https://raw.githubusercontent.com/bitzl/vst-sine/master/images/64bit-spectrum.png)	

![Waveform](https://raw.githubusercontent.com/bitzl/vst-sine/master/images/64bit-waveform.png)


# Integer

In this approach we count samples and calculate the time for each sample. Accumulation of floating point errors over time is not possible. The result looks and sounds as good as in the `f64` test:

![Spectrum](https://raw.githubusercontent.com/bitzl/vst-sine/master/images/int-spectrum.png)	

![Waveform](https://raw.githubusercontent.com/bitzl/vst-sine/master/images/int-waveform.png)

