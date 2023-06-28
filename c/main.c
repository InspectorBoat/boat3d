#define GLFW_INCLUDE_VULKAN
#define WIDTH 600
#define HEIGHT 600
#include <GLFW\glfw3.h>
#include <stdio.h>
#include <stdlib.h>

typedef uint8_t  u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t  i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

typedef float  f32;
typedef double f64;

typedef int isize;
typedef unsigned int usize;

typedef void unknown;
typedef void none;

typedef struct {
	u32 graphics;
	u32 other;
} QueueFamilyIndices;

GLFWwindow *createWindow() {
	glfwInit();
	glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
	return glfwCreateWindow(WIDTH, HEIGHT, "boat3d", NULL, NULL);
}

VkBool32 debugCallback(
		VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity,
    	VkDebugUtilsMessageTypeFlagsEXT messageType,
    	const VkDebugUtilsMessengerCallbackDataEXT *pCallbackData,
    	unknown *pUserData
	) {
	printf((*pCallbackData).pMessage);
	printf("\n");
	return 0;
}

VkInstance createVulkanInstance() {
	VkApplicationInfo appInfo = {
		sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
		pApplicationName: "boat3d",
		applicationVersion: VK_MAKE_VERSION(1, 0, 0),
		pEngineName: "boat3d",
		engineVersion: VK_MAKE_VERSION(1, 0, 0),
		apiVersion: VK_API_VERSION_1_0
	};
	
	VkDebugUtilsMessengerCreateInfoEXT debugCreateInfo = {
		sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
		messageSeverity: VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
		messageType: VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
		pfnUserCallback: debugCallback,
		pUserData: NULL
	};

	uint32_t glfwExtensionCount;
	const char **glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
	
	VkInstanceCreateInfo instanceCreateInfo = {
		sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
		pNext: &debugCreateInfo,
		pApplicationInfo: &appInfo,
		enabledExtensionCount: glfwExtensionCount,
		ppEnabledExtensionNames: glfwExtensions,
		enabledLayerCount: 1,
	};
	
	const char *name = "VK_LAYER_KHRONOS_validation";
	instanceCreateInfo.ppEnabledLayerNames = &name;
	
	VkInstance instance;

	if (vkCreateInstance(&instanceCreateInfo, NULL, &instance)) {
		printf("failed to create instance");
	}

	return instance;
}

VkPhysicalDevice pickPhysicalDevice(VkInstance *instance) {
	u32 physicalDeviceCount = 0;
	vkEnumeratePhysicalDevices(*instance, &physicalDeviceCount, NULL);
	VkPhysicalDevice *devices = calloc(physicalDeviceCount, sizeof(VkPhysicalDevice));
	vkEnumeratePhysicalDevices(*instance, &physicalDeviceCount, devices);
	VkPhysicalDevice device = *devices;
	free(devices);
	return device;
}

QueueFamilyIndices findQueueFamilies(VkPhysicalDevice physicalDevice) {
	u32 propertyCount;
	vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &propertyCount, NULL);
	VkQueueFamilyProperties *properties = malloc(sizeof(VkQueueFamilyProperties) * propertyCount);
	vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &propertyCount, properties);
	QueueFamilyIndices indices;

	for (usize i = 0; i < propertyCount; i ++) {
		VkQueueFamilyProperties property = *(properties + i);
		if (property.queueFlags & VK_QUEUE_GRAPHICS_BIT) indices.graphics = i;
	}
	return indices;
}

VkDevice createLogicalDevice(VkPhysicalDevice physicalDevice) {
	QueueFamilyIndices indices = findQueueFamilies(physicalDevice);
	f32 queuePriority = 1.0;
	VkDeviceQueueCreateInfo queueCreateInfo = {
		sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
		queueFamilyIndex: indices.graphics,
		queueCount: 1,
		pQueuePriorities: &queuePriority
	};
	VkPhysicalDeviceFeatures featureInfo = {};
	VkDeviceCreateInfo deviceCreateInfo = {
		sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
		pQueueCreateInfos: &queueCreateInfo,
		queueCreateInfoCount: 1,
		pEnabledFeatures: &featureInfo
	};
	VkDevice device;
	printf("%i", vkCreateDevice(physicalDevice, &deviceCreateInfo, NULL, &device) == VK_SUCCESS);
	VkQueue graphicsQueue;
	vkGetDeviceQueue(device, indices.graphics, 0, &graphicsQueue);
	return device;
}

VkSurfaceKHR createSurface(VkInstance instance, GLFWwindow *window) {
	VkSurfaceKHR surface;
	printf("%i", glfwCreateWindowSurface(instance, window, NULL, &surface) == VK_SUCCESS);
}

int main() {
	GLFWwindow *window = createWindow();
	VkInstance instance = createVulkanInstance();
	VkSurfaceKHR surface = createSurface(instance, window);
	VkPhysicalDevice physicalDevice = pickPhysicalDevice(&instance);
	VkDevice device = createLogicalDevice(physicalDevice);	
	while (!glfwWindowShouldClose(window)) glfwPollEvents();

	return 0;
}
