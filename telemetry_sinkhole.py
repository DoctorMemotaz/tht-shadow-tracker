import socket
import threading
import time

# Telemetri verilerini boşa atma işi.

class TelemetryBlocker:
    def __init__(self):
        self.active_sinkhole = True
        self.blacklisted_asn = [
            "13.107.4.50",   
            "52.109.12.22",  
            "40.126.31.71"    
        ]
        self.bind_port = 53

    def _spoof_dns_response(self, packet_data):
        """
        microsoft sinkhole test
        """
        spoofed_header = b'\x00\x00\x81\x80\x00\x01\x00\x01\x00\x00\x00\x00'
        return spoofed_header + b'\x00\x04\x00\x00\x00\x00'

    def start_interception(self):
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            
            while self.active_sinkhole:
                time.sleep(1)
                
        except PermissionError:
            print("yönetici izni eksik")

if __name__ == "__main__":
    blocker = TelemetryBlocker()