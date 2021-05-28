<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateEmployeeDetail</name>
   <tag></tag>
   <elementGuidId>f602b8c8-ab34-46e5-adcc-0baab5515ed4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;1&quot;
    },
    {
      &quot;name&quot;: &quot;firstName&quot;,
      &quot;value&quot;: &quot;Pauli&quot;
    },
    {
      &quot;name&quot;: &quot;middleName&quot;,
      &quot;value&quot;: &quot;Mirco&quot;
    },
    {
      &quot;name&quot;: &quot;lastName&quot;,
      &quot;value&quot;: &quot;Maldana&quot;
    },
    {
      &quot;name&quot;: &quot;code&quot;,
      &quot;value&quot;: &quot;3333&quot;
    },
    {
      &quot;name&quot;: &quot;dob&quot;,
      &quot;value&quot;: &quot;1991-07-06&quot;
    },
    {
      &quot;name&quot;: &quot;licenseNumber&quot;,
      &quot;value&quot;: &quot;1234567890&quot;
    },
    {
      &quot;name&quot;: &quot;licenseNumberExpDate&quot;,
      &quot;value&quot;: &quot;2021-07-06&quot;
    },
    {
      &quot;name&quot;: &quot;maritalStatus&quot;,
      &quot;value&quot;: &quot;Single&quot;
    },
    {
      &quot;name&quot;: &quot;gender&quot;,
      &quot;value&quot;: &quot;M-Male&quot;
    },
    {
      &quot;name&quot;: &quot;otherId&quot;,
      &quot;value&quot;: &quot;SIM&quot;
    },
    {
      &quot;name&quot;: &quot;nationality&quot;,
      &quot;value&quot;: &quot;Indonesian&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 73e120bc28deb814c7d8c7a0acc368c20a9c6950</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${base_url}/api/v1/employee/:id</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.base_url</defaultValue>
      <description></description>
      <id>ffa54f34-5c70-4f49-ae9b-b2a0397b8909</id>
      <masked>false</masked>
      <name>base_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
