#!/usr/bin/python3

import xml.etree.ElementTree as ET
import os, shutil, subprocess, sys

from svdwash import (clusterfy, deprefix, peripheral_derivatives,
                     output_and_generate, register_array, register_derivatives)

SVD2RUST='/home/mirror/svd2rust/target/debug/svd2rust'

scriptdir = os.path.dirname(sys.argv[0])
if scriptdir != '':
    os.chdir(scriptdir)

svd = ET.parse('STM32G030.svd')
assert svd is not None

alternates_remove = set()
alternates_keep = {}

deprefix(svd, alternates_remove, alternates_keep)

dma = svd.find(".//peripheral[name='DMA1']")
assert dma is not None
clusterfy(dma, 'CH[%s]', ['CR', 'NDTR', 'PAR', 'MAR'],
          [f'CCR{i} CNDTR{i} CPAR{i} CMAR{i}'.split() for i in range(1, 8)])

dmamux = svd.find(".//peripheral[name='DMAMUX']")
assert dmamux is not None
register_array(dmamux, 'C0CR', 'CCR[%s]', [f'C{i}CR' for i in range(12)]);

tamp = svd.find(".//peripheral[name='TAMP']")
assert tamp is not None
register_array(tamp, 'BKP0R', 'BKPR[%s]', [f'BKP{i}R' for i in range(9)])

#pwr = svd.find(".//peripheral[name='PWR']")
#register_derivatives(pwr, 'PUCRA', ['PUCRB', 'PUCRC'])
#register_derivatives(pwr, 'PDCRA', ['PDCRB', 'PDCRC'])

#peripheral_derivatives(svd, 'GPIOA', ['GPIOB', 'GPIOC'])

output_and_generate(svd)
