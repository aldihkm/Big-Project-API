import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.RequestObject

response1 = WS.sendRequest(findTestObject('Object Repository/Cilsy Big Project/Get Token'))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(response1.getResponseBodyContent())

def token = result.access_token

println(token)

GlobalVariable.token = token

def request = (RequestObject)findTestObject('Cilsy Big Project/Employee/14 Save Employee Job Detail')

ArrayList<TestObjectProperty> HTTPHeader = new ArrayList <TestObjectProperty>()

HTTPHeader.add(new TestObjectProperty('Content-Type', ConditionType.EQUALS, 'application/x-www-form-urlencoded'))

HTTPHeader.add(new TestObjectProperty('Authorization', ConditionType.EQUALS, "Bearer" + token))

request.setHttpHeaderProperties(HTTPHeader)

response = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/14 Save Employee Job Detail', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response, 200)


def request2 = (RequestObject)findTestObject('Cilsy Big Project/Employee/15 Update Employee Job Detail')

ArrayList<TestObjectProperty> HTTPHeader2 = new ArrayList <TestObjectProperty>()

HTTPHeader2.add(new TestObjectProperty('Content-Type', ConditionType.EQUALS, 'application/x-www-form-urlencoded'))

HTTPHeader2.add(new TestObjectProperty('Authorization', ConditionType.EQUALS, "Bearer" + token))

request.setHttpHeaderProperties(HTTPHeader)

response2 = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/15 Update Employee Job Detail', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response, 200)


def request3 = (RequestObject)findTestObject('Cilsy Big Project/Employee/13 Employee Job Detail')

ArrayList<TestObjectProperty> HTTPHeader3 = new ArrayList <TestObjectProperty>()

HTTPHeader3.add(new TestObjectProperty('Authorization', ConditionType.EQUALS, "Bearer" + token))

request.setHttpHeaderProperties(HTTPHeader)

response3 = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/13 Employee Job Detail', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response, 200)