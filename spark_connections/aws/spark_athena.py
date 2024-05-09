from decouple import config
from pyspark.sql import SparkSession
from rich import print

import timeit
import warnings

warnings.filterwarnings("ignore")

# Configurações da AWS
REGION = 'us-west-2'

# Caminhos dos JARs necessários
athena_jar = '/home/dgamorim/labs/AthenaJDBC42-2.0.33.jar'
aws_pkg = 'com.amazonaws:aws-java-sdk:1.12.717'

# Configuração da sessão Spark
spark = (
    SparkSession
    .builder
    .config('spark.jars', athena_jar)
    .config("spark.jars.packages", aws_pkg)
    .getOrCreate()
)

query = "SELECT * FROM brasil_io.salarios_magisterio limit 1000000"

# Marca o tempo de início
start = timeit.default_timer()
athena_df = (
    spark.read.format("jdbc")
    .option("driver","com.simba.athena.jdbc.Driver")
    .option("url", f"jdbc:awsathena://athena.{REGION}.amazonaws.com:443")
    .option("S3OutputLocation", config('AWS_BUCKET_OREGON') + "athena_oregon")
    .option("query", query)
    .option("user",config('AWS_KEY_ID'))
    .option("password",config('AWS_SECRET_KEY'))
    .load()
)

spark.sparkContext.setLogLevel("WARN")

athena_df.show(10)
end = timeit.default_timer()
tempo_execucao = end - start
print("Tempo de execução:", tempo_execucao, "segundos")

# ? INFORMAÇÔES
# ? export SPARK_LOCAL_IP=172.20.60.159
# ? .config("spark.logLevel", "ERROR")
# ? .config("spark.executor.instances", "10")
# ? .config("spark.executor.cores", "4")
# ? .config("spark.driver.memory", "2g")
# ? .config("spark.executor.memory", "2g")
# ? .config("spark.default.parallelism", "200")
# spark-submit \
#   --master local[4] \   # Roda localmente com 4 cores
#   --deploy-mode client \
#   --jars /caminho/para/aws-java-sdk.jar,/caminho/para/AthenaJDBC.jar \
#   /caminho/para/meu_script.py \
#   arg1 arg2

