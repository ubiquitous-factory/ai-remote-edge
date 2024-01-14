int a0 = 0;
float r1 = 10000.;
float a = 0.001129148;
float b = 0.000234125;
float c = 0.0000000876741;

void setup()
{
  Serial.begin(57600);
}
void loop()
{

  int v = analogRead(a0);
  float r2 = r1 * (1024.0 / v - 1);
  float log_r2 = log(r2);
  float temp_k = 1.0 / (a + b * log_r2 + c * log_r2 * log_r2 * log_r2); //  Temp Kelvin
  float temp_c = temp_k - 273.15;                                       // Convert Kelvin to Celcius
  float temp_f = (temp_c * 9.0) / 5.0 + 32.0;                           // Convert Celcius to Fahrenheit

  Serial.print(v);
  Serial.print(" A0; ");
  Serial.print(temp_k);
  Serial.print(" K; ");
  Serial.print(temp_c);
  Serial.print(" C; ");
  Serial.print(temp_f);
  Serial.println(" F ");
  delay(500);
}