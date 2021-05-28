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

response = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/12 Save Employee Dependents', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response, 200)

def slurper2 = new groovy.json.JsonSlurper()

def result2 = slurper.parseText(response.getResponseBodyContent())

def seqIdDependents = result2.sequenceNumber

println(seqIdDependents)

GlobalVariable.seqIdDependents = seqIdDependents

response2 = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/09 Employee Dependents', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response2, 200)

response3 = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/11 Update Dependents', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response3, 200)

def request4 = (RequestObject)findTestObject('Cilsy Big Project/Employee/10 Delete Employee Dependents')

response4 = WS.sendRequestAndVerify(findTestObject('Cilsy Big Project/Employee/10 Delete Employee Dependents', (['id' : GlobalVariable.id])))

WS.verifyResponseStatusCode(response4, 200)